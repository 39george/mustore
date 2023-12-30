use std::sync::Arc;

use deadpool_postgres::Client;
use fred::clients::RedisClient;
use fred::interfaces::HashesInterface;
use fred::interfaces::KeysInterface;
use fred::types::Scanner;
use futures::TryStreamExt;
use time::OffsetDateTime;
use tokio_cron_scheduler::Job;
use tokio_cron_scheduler::JobScheduler;
use tokio_cron_scheduler::JobSchedulerError;
use tracing::Instrument;

use crate::cornucopia::queries::internal;
use crate::service_providers::object_storage::ObjectStorage;
use crate::startup::MOSKOW_TIME_OFFSET;

pub fn update_available_songs_materialized_view_task(
    pg_client: Arc<Client>,
) -> Result<Job, JobSchedulerError> {
    let job = Job::new_async("0 0 * * * *", move |uuid, l| {
        let pg_client = pg_client.clone();
        let span = tracing::info_span!("Refresh available songs task", %uuid);
        Box::pin(async move {
            match internal::refresh_available_songs().bind(&*pg_client).await {
                Ok(rows) => {
                    tracing::info!("Successfully refreshed available songs materialized view, updated rows: {rows}");
                }
                Err(e) => {
                    tracing::error!("Failed to refresh available songs materialized view: {e}");
                }
            }
            next_time_for_job(l, uuid).await;
        }.instrument(span))
    })?;
    Ok(job)
}

pub fn check_current_user_uploads(
    object_storage: Arc<ObjectStorage>,
    client: Arc<RedisClient>,
) -> Result<Job, JobSchedulerError> {
    let job = Job::new_async("0 1 * * * *", move |uuid, l| {
        let span =
            tracing::info_span!("Check outdated uploads in redis task", %uuid);
        let client = client.clone();
        let obj_storage = object_storage.clone();
        Box::pin(
            async move {
                let pattern = "upload_request*";
                let mut scan = client.scan(pattern, None, None);
                while let Ok(Some(mut page)) = scan.try_next().await {
                    if let Some(keys) = page.take_results() {
                        for key in keys.into_iter() {
                            let date: String = match client.get(&key).await {
                                Ok(d) => d,
                                Err(e) => {
                                    tracing::error!("Failed to get timestamp from upload request: {e}");
                                    continue;
                                }
                            };
                            let created_at = OffsetDateTime::parse(
                                &date,
                                &crate::DEFAULT_TIME_FORMAT,
                            )
                            .unwrap();
                            let now = OffsetDateTime::now_utc();
                            if (now - created_at) > time::Duration::HOUR {
                                match client.del::<u32, &fred::types::RedisKey>(&key).await {
                                    Ok(count) => {
                                        tracing::info!("{:?} is outdated, and deleted", key);
                                        if count != 1 {
                                            tracing::error!("Strange deletion result, should be 1, but got {count}");
                                        }
                                    }
                                    Err(e) => { tracing::error!("Failed to delete key from redis: {e}"); continue; },
                                }
                                if let Some(key) = key.into_string() {
                                    match obj_storage.delete_object_by_key(&key).await {
                                        Ok(()) => tracing::info!("Object with key {key} is successfully deleted from obj storage"),
                                        Err(e) => tracing::warn!("Failed to delete object with key {key} from object storage: {e}"),
                                    }
                                }
                            }
                        }
                    }
                    if let Err(e) = page.next() {
                        tracing::error!("Failed to get next page: {e}");
                        break;
                    }
                }
                next_time_for_job(l, uuid).await;
            }
            .instrument(span),
        )
    })?;
    Ok(job)
}

async fn next_time_for_job(mut l: JobScheduler, uuid: uuid::Uuid) {
    match l.next_tick_for_job(uuid).await {
        Ok(Some(ts)) => {
            if let Ok(utc_target) =
                time::OffsetDateTime::from_unix_timestamp(ts.timestamp())
            {
                let time = utc_target.to_offset(MOSKOW_TIME_OFFSET.clone());
                tracing::info!(
                        "Next time for available songs materialized view update: {:?}",
                        time
                    );
            }
        }
        _ => {
            tracing::warn!("Could not get next tick for 1h job")
        }
    }
}

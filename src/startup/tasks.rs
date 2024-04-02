use std::sync::Arc;

use deadpool_postgres::Client;
use fred::clients::RedisClient;
use time::OffsetDateTime;
use tokio_cron_scheduler::Job;
use tokio_cron_scheduler::JobScheduler;
use tokio_cron_scheduler::JobSchedulerError;
use tracing::Instrument;

use crate::cornucopia::queries::internal;
use crate::domain::upload_request::remove_outdated_uploads_from_redis;
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
            next_time_for_job(l, uuid, "available songs materialized view update").await;
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
                let now = OffsetDateTime::now_utc();
                let removed_keys = remove_outdated_uploads_from_redis(&client, |created_at| (now - created_at) > time::Duration::HOUR).await;
                for key in removed_keys {
                    match key.parse() {
                        Ok(key) => {
                            match obj_storage.delete_object_by_key(&key).await {
                                Ok(()) => tracing::info!("Object with key {key} is successfully deleted from obj storage"),
                                Err(e) => tracing::warn!("Failed to delete object with key {key} from object storage: {e}"),
                            }
                        },
                        Err(e) => {
                            tracing::error!("Failed to parse {key} as objet key: {e}");
                        },
                    }
                }
                next_time_for_job(l, uuid, "users uploads check").await;
            }
            .instrument(span),
        )
    })?;
    Ok(job)
}

async fn next_time_for_job(mut l: JobScheduler, uuid: uuid::Uuid, job: &str) {
    match l.next_tick_for_job(uuid).await {
        Ok(Some(ts)) => {
            if let Ok(utc_target) =
                time::OffsetDateTime::from_unix_timestamp(ts.timestamp())
            {
                let time = utc_target.to_offset(MOSKOW_TIME_OFFSET.clone());
                tracing::info!("Next time for {job}: {:?}", time);
            }
        }
        _ => {
            tracing::warn!("Could not get next tick for job")
        }
    }
}

use axum::http::Request;
use axum::http::Response;
use futures::future::BoxFuture;
use std::fmt::Display;
use std::task::Context;
use std::task::Poll;
use tower::Layer;
use tower::Service;

#[derive(Clone)]
pub struct LogService<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for LogService<S>
where
    S: Service<Request<B>, Response = Response<B>> + Send + 'static,
    B: Send + 'static + std::fmt::Debug,
    S::Future: Send + 'static,
    S::Error: Display + std::fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>; // use `BoxFuture`

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let fut = self.inner.call(req);
        Box::pin(async move {
            let result = fut.await;
            match &result {
                Ok(res) if res.status().is_success() => {
                    println!("----------Info: Response was OK----------");
                }
                Ok(res)
                    if res.status().is_client_error()
                        || res.status().is_server_error() =>
                {
                    println!(
                        "----------Error: Response was an ERROR----------"
                    );
                    let body = res.body();
                    println!("ERROR: {}, {:?}", res.status(), body);
                }
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Error: {e}");
                }
            }
            result
        })
    }
}

#[derive(Clone)]
pub struct LogLayer;

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogService { inner }
    }
}

pub mod map_response {
    use axum::{body::Body, extract::Request, response::Response};
    use axum::{body::Bytes, http::StatusCode};
    use futures::future::BoxFuture;
    use http_body_util::BodyExt;
    use std::task::{Context, Poll};
    use tower::{Layer, Service};

    #[derive(Clone)]
    pub struct BadRequestIntoJsonLayer;

    impl<S> Layer<S> for BadRequestIntoJsonLayer {
        type Service = BadRequestInJson<S>;

        fn layer(&self, inner: S) -> Self::Service {
            BadRequestInJson { inner }
        }
    }

    #[derive(Clone)]
    pub struct BadRequestInJson<S> {
        inner: S,
    }

    impl<S> Service<Request> for BadRequestInJson<S>
    where
        S: Service<Request, Response = Response> + Send + 'static,
        S::Future: Send + 'static,
    {
        type Response = S::Response;
        type Error = S::Error;
        // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
        type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

        fn poll_ready(
            &mut self,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, request: Request) -> Self::Future {
            let future = self.inner.call(request);
            Box::pin(async move {
                let response: Response = future.await?;
                let (mut parts, body) = response.into_parts();
                let body = if parts.status == StatusCode::BAD_REQUEST {
                    let bytes = match buffer(body).await {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            tracing::error!("Error: {e}");
                            Bytes::new()
                        }
                    };
                    let (len, body) =
                        if let Ok(msg) = std::str::from_utf8(&bytes) {
                            let msg = format!("{{\"caused_by\": \"{}\"}}", msg);
                            (msg.bytes().len(), Body::from(msg))
                        } else {
                            (bytes.len(), Body::from(bytes))
                        };
                    parts
                        .headers
                        .insert(http::header::CONTENT_LENGTH, len.into());
                    parts.headers.insert(
                        http::header::CONTENT_TYPE,
                        http::HeaderValue::from_static(
                            "application/json; charset=utf-8",
                        ),
                    );
                    body
                } else {
                    body
                };
                let response = Response::from_parts(parts, body);
                Ok(response)
            })
        }
    }

    async fn buffer<B>(body: B) -> Result<Bytes, String>
    where
        B: axum::body::HttpBody<Data = Bytes>,
        B::Error: std::fmt::Display,
    {
        let bytes = match body.collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(err) => {
                return Err(format!("failed to read body: {err}"));
            }
        };

        Ok(bytes)
    }
}

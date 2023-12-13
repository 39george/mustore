use axum::http::Request;
use axum::http::Response;
use futures::future::BoxFuture;
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
                    println!("ERROR: {}, {:?}", res.status(), res.body());
                }
                _ => {}
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

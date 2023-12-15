/// Can insert middleware like this:
/// ```ignore
/// .layer(ServiceBuilder::new()
///     .layer_fn(|inner| ErrorLoggingMiddleware::new(inner))
/// )
///```
mod example_middleware {

    use axum::http::{Request, Response};
    use std::convert::Infallible;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tower::Service;

    #[derive(Clone)]
    // `S` is a `Service`
    struct ErrorLoggingMiddleware<S> {
        inner: S,
    }

    impl<S, B> Service<Request<B>> for ErrorLoggingMiddleware<S>
    where
        S: Service<Request<B>, Response = Response<B>, Error = Infallible>
            + Clone
            + Send
            + 'static,
        S::Future: Send,
        B: Send + 'static + std::fmt::Debug,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = Pin<
            Box<
                dyn Future<Output = Result<Self::Response, Self::Error>> + Send,
            >,
        >;

        fn poll_ready(
            &mut self,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, req: Request<B>) -> Self::Future {
            println!("AAA {:?}", &req);
            let fut = self.inner.call(req);

            // Box the future to return
            Box::pin(async move {
                let result = fut.await;
                if let Err(ref e) = result {
                    // Here, log your error as needed
                    println!("Error processing the request: {:?}", e);
                }
                result
            })
        }
    }
}

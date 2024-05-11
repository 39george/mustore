use futures::Future;
use std::pin::Pin;
use std::sync::OnceLock;

pub struct WebDriver {
    pub tx: tokio::sync::mpsc::Sender<(
        Box<
            dyn FnOnce(fantoccini::Client) -> Pin<Box<dyn Future<Output = ()>>>
                + Send,
        >,
        tokio::sync::oneshot::Sender<()>,
    )>,
    deinit_tx: tokio::sync::oneshot::Sender<()>,
}

impl WebDriver {
    pub fn new() -> Self {
        let (deinit_tx, _deinit_rx) = tokio::sync::oneshot::channel();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(
            Box<
                dyn FnOnce(
                        fantoccini::Client,
                    )
                        -> Pin<Box<dyn Future<Output = ()>>>
                    + Send,
            >,
            tokio::sync::oneshot::Sender<()>,
        )>(100);
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let client = fantoccini::ClientBuilder::native()
                    .connect("http://localhost:4444")
                    .await
                    .expect("failed to connect to WebDriver");
                while let Some((task, tx)) = rx.recv().await {
                    task(client.clone()).await;
                    tx.send(()).unwrap();
                }
                client.close().await.unwrap();
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            });
        });
        Self { tx, deinit_tx }
    }
    pub async fn send<F, Fut>(&self, f: F)
    where
        F: FnOnce(fantoccini::Client) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx
            .send((Box::new(move |client| Box::pin(f(client))), tx))
            .await
            .expect("Failed to send task over channel");
        rx.await.unwrap();
    }
}

impl Drop for WebDriver {
    fn drop(&mut self) {
        println!("Dropping webdriver");
    }
}

pub fn webdriver() -> &'static WebDriver {
    static DRIVER: OnceLock<WebDriver> = OnceLock::new();
    DRIVER.get_or_init(|| WebDriver::new())
}

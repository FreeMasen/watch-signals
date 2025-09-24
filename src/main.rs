use std::time::Duration;

use tokio::signal::unix::SignalKind;

static SIGNALS: &[usize] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 31,
];

#[tokio::main]
async fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();
    let (tx, mut rx) = tokio::sync::mpsc::channel(1024);
    for signal in 0..50 {
        let Ok(mut sig) = tokio::signal::unix::signal(SignalKind::from_raw(signal as _)) else {
            log::info!("signal {signal} is invalid, skipping");
            continue;
        };
        let tx = tx.clone();
        tokio::task::spawn(async move {
            loop {
                sig.recv().await;
                log::info!("signal {signal} was received");
                if tx.send(signal).await.is_err() {
                    break;
                }
            }
        });
    }
    while let Some(signal) = rx.recv().await {
        log::info!("signal {signal} was processes");
        if signal == SignalKind::interrupt().as_raw_value() {
            log::info!("sigint, exiting in 1 second");
            tokio::time::sleep(Duration::from_secs(1)).await;
            break;
        }
    }
}

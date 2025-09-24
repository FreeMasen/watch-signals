use std::time::Duration;

use tokio::signal::unix::SignalKind;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();
    let (tx, mut rx) = tokio::sync::mpsc::channel(1024);
    for signal in 0..65 {
        let Ok(mut sig) = tokio::signal::unix::signal(SignalKind::from_raw(signal as _)) else {
            log::info!("signal {} is invalid, skipping", sig_name(signal));
            continue;
        };
        let tx = tx.clone();
        tokio::task::spawn(async move {
            loop {
                sig.recv().await;
                log::info!("signal {} was received", sig_name(signal));
                if tx.send(signal).await.is_err() {
                    break;
                }
            }
        });
    }
    while let Some(signal) = rx.recv().await {
        log::info!("signal {} was processes", sig_name(signal));
        if signal == SignalKind::interrupt().as_raw_value()
            || signal == SignalKind::terminate().as_raw_value()
            || signal == SignalKind::quit().as_raw_value()
            || signal == SignalKind::alarm().as_raw_value()
            || signal == 6 // SIGABRT
        {
            log::info!("exiting in 1 second");
            tokio::time::sleep(Duration::from_secs(1)).await;
            break;
        }
    }
}

fn sig_name(sig: i32) -> &'static str {
    match sig {
        1 => "SIGHUP",
        2 => "SIGINT",
        3 => "SIGQUIT",
        4 => "SIGILL",
        5 => "SIGTRAP",
        6 => "SIGABRT",
        7 => "SIGBUS",
        8 => "SIGFPE",
        9 => "SIGKILL",
        10 => "SIGUSR1",
        11 => "SIGSEGV",
        12 => "SIGUSR2",
        13 => "SIGPIPE",
        14 => "SIGALRM",
        15 => "SIGTERM",
        16 => "SIGSTKFLT",
        17 => "SIGCHLD",
        18 => "SIGCONT",
        19 => "SIGSTOP",
        20 => "SIGTSTP",
        21 => "SIGTTIN",
        22 => "SIGTTOU",
        23 => "SIGURG",
        24 => "SIGXCPU",
        25 => "SIGXFSZ",
        26 => "SIGVTALRM",
        27 => "SIGPROF",
        28 => "SIGWINCH",
        29 => "SIGIO",
        30 => "SIGPWR",
        31 => "SIGSYS",
        32 => "UNKNOWN-32",
        33 => "UNKNOWN-33",
        34 => "SIGRTMIN",
        35 => "SIGRTMIN+1",
        36 => "SIGRTMIN+2",
        37 => "SIGRTMIN+3",
        38 => "SIGRTMIN+4",
        39 => "SIGRTMIN+5",
        40 => "SIGRTMIN+6",
        41 => "SIGRTMIN+7",
        42 => "SIGRTMIN+8",
        43 => "SIGRTMIN+9",
        44 => "SIGRTMIN+10",
        45 => "SIGRTMIN+11",
        46 => "SIGRTMIN+12",
        47 => "SIGRTMIN+13",
        48 => "SIGRTMIN+14",
        49 => "SIGRTMIN+15",
        50 => "SIGRTMAX-14",
        51 => "SIGRTMAX-13",
        52 => "SIGRTMAX-12",
        53 => "SIGRTMAX-11",
        54 => "SIGRTMAX-10",
        55 => "SIGRTMAX-9",
        56 => "SIGRTMAX-8",
        57 => "SIGRTMAX-7",
        58 => "SIGRTMAX-6",
        59 => "SIGRTMAX-5",
        60 => "SIGRTMAX-4",
        61 => "SIGRTMAX-3",
        62 => "SIGRTMAX-2",
        63 => "SIGRTMAX-1",
        64 => "SIGRTMAX",
        _ => "UNKNOWN-65+",
    }
}

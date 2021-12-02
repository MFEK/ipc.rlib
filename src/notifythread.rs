use log;
use notify::{self, op as fsop, RawEvent, Watcher as _};

use std::path;
use std::sync::mpsc::{channel, Sender};
use std::thread;

fn launch_impl(dir: path::PathBuf, tx_parent: Sender<path::PathBuf>) {
    let (tx, rx) = channel();

    let mut watcher = notify::raw_watcher(tx).unwrap();
    watcher
        .watch(&dir, notify::RecursiveMode::Recursive)
        .unwrap();
    log::trace!(
        "Launched notify::RawWatcher in recursive mode on {:?}",
        &dir
    );

    loop {
        let recv = rx.recv();
        match recv {
            Ok(RawEvent {
                path,
                op: Ok(fsop::CLOSE_WRITE),
                ..
            }) => {
                if let Some(path) = path {
                    log::info!("Filesystem write event: {:?}", path);
                    tx_parent.send(path).unwrap();
                } else {
                    log::error!("Got a filesystem write without a path?");
                }
            }
            Ok(RawEvent { op: Ok(_), .. }) => {
                log::debug!("Filesystem event: {:?}", recv)
            }
            Ok(event) => log::error!("Broken filesystem event: {:?}", event),
            Err(e) => log::error!("Filesystem watcher error: {:?}", e),
        }
    }
}

pub fn launch(dir: path::PathBuf, tx: Sender<path::PathBuf>) {
    thread::spawn(|| launch_impl(dir, tx));
}

use log;
use notify::{self, Event, Watcher as _, EventKind};

use std::path;
use std::sync::mpsc::{channel, Sender};
use std::thread;

#[rustfmt::skip]
fn launch_impl(dir: path::PathBuf, tx_parent: Sender<path::PathBuf>) {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx).unwrap();
    match watcher.watch(&dir, notify::RecursiveMode::Recursive) {
        Ok(()) => (),
        Err(notify::Error{kind: notify::ErrorKind::Io(e), ..}) => {
            // log::error! differentiates between static str and literal str
            macro_rules! LAUNCHFAILMSG {
                () => ("Cannot launch filesystem watch thread! {}. Won't receive important events!")
            }
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                log::error!(LAUNCHFAILMSG!(), format!("No permissions on target {:?}", &dir));
            } else {
                log::error!(LAUNCHFAILMSG!(), format!("I/O error: {:?}", e));
            }
            return;
        }
        Err(e) => {
            log::error!("Cannot launch filesystem watch thread! Unknown error: {:?}", e);
            return;
        }
    }

    log::trace!("Launched notify::RawWatcher in recursive mode on {:?}", &dir);

    loop {
        let recv = rx.recv();
        match recv {
            Ok(Ok(Event { paths, kind: EventKind::Modify(_) | EventKind::Create(_), .. })) => {
                if paths.len() <= 0 {
                    log::error!("Got a filesystem write without a path?");
                }
                for path in paths {
                    log::info!("Filesystem write event: {:?}", path);
                    tx_parent.send(path).unwrap();
                }
            }
            Ok(Ok(event)) => {
                log::debug!("Filesystem event: {:?}", &event)
            }
            Ok(Err(e)) => {
                log::error!("Error in watcher!: {:?}", e)
            }
            Err(e) => {
                log::error!("Error in recv, breaking!: {:?}", e);
                break
            }
        }
    }
}

pub fn launch(dir: path::PathBuf, tx: Sender<path::PathBuf>) {
    log::trace!("Spawning fsnotify thread on {:?}; tx {:?}", &dir, &tx);
    thread::spawn(|| launch_impl(dir, tx));
}

// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::thread;

pub fn spawn<F>(name: &'static str, f: F) -> thread::JoinHandle<()>
where
    F: 'static + Send + FnOnce() -> nostr_sdk::Result<()>,
{
    thread::Builder::new()
        .name(name.to_owned())
        .spawn(move || {
            if let Err(e) = f() {
                log::warn!("{} thread failed: {}", name, e);
            }
        })
        .expect("failed to spawn a thread")
}

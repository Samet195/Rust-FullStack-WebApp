#![cfg(target_os = "android")]

use std::{sync::mpsc, thread::spawn};

use crate::server;

use actix_web::{dev::ServerHandle, rt::System};
use jni_mangle::mangle;
use once_cell::sync::OnceCell;

static HANDLER: OnceCell<ServerHandle> = OnceCell::new();

#[mangle(package = "backend", class = "Server", alias = false)]
pub fn start() -> u16 {
    #[cfg(debug_assertions)]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );

    let (tx_port, rx_port) = mpsc::channel();
    let (tx_handle, rx_handle) = mpsc::channel();

    spawn(|| System::new().block_on(server::run(tx_port, Some(tx_handle))));

    let _ = HANDLER.set(rx_handle.recv().unwrap());

    rx_port.recv().unwrap()
}

#[mangle(package = "backend", class = "Server", alias = false)]
pub fn pause() {
    spawn(|| System::new().block_on(HANDLER.get().unwrap().pause()));
}

#[mangle(package = "backend", class = "Server", alias = false)]
pub fn resume() {
    spawn(|| System::new().block_on(HANDLER.get().unwrap().resume()));
}

#[mangle(package = "backend", class = "Server", alias = false)]
pub fn stop() {
    spawn(|| System::new().block_on(HANDLER.get().unwrap().stop(true)));
}

use std::sync::mpsc;
use std::thread;

use actix_web::rt::System;
use anyhow::Result;

use backend::server;

use utils::*;
mod utils;

fn main() -> Result<()> {
    init_logger();
    init_i18n();

    let trx = mpsc::channel();
    let th = thread::spawn(move || System::new().block_on(server::run(trx.0, None)));

    #[cfg(any(feature = "open_browser", not(feature = "internal_port")))]
    let port = trx.1.recv()?;

    #[cfg(not(feature = "internal_port"))]
    print_log(port);

    #[cfg(all(feature = "open_browser", not(feature = "internal_port"),))]
    if is_open_browser() {
        open_browser(port);
    }

    Ok(th.join().unwrap()?)
}

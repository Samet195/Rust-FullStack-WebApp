use common::l10n::detect_locale;
use std::env;

pub fn init_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    pretty_env_logger::init();
}

pub fn init_i18n() {
    let lang = detect_locale().unwrap_or("en_US".into());
    rust_i18n::set_locale(lang.as_str());
}

#[cfg(any(feature = "open_browser", not(feature = "internal_port")))]
#[inline]
pub fn get_protocol() -> String {
    if cfg!(feature = "tls") {
        "https"
    } else {
        "http"
    }
    .to_string()
}

#[cfg(any(feature = "open_browser", not(feature = "internal_port")))]
#[inline]
pub fn get_addr() -> String {
    let config = common::Config::load_from_embeded().unwrap_or_default();
    if cfg!(feature = "internal_port") {
        config.addr.internal
    } else {
        config.addr.public
    }
    .to_string()
}

#[cfg(not(feature = "internal_port"))]
#[inline]
pub fn print_log(port: u16) {
    let protocol = get_protocol();
    let addr = get_addr();

    log::info!("Starting HTTP server at {}://{}:{}/", protocol, addr, port);
}

#[cfg(feature = "open_browser")]
#[inline]
pub fn open_browser(port: u16) {
    let protocol = get_protocol();
    let addr = get_addr();

    log::info!("Address opening in default browser..");

    std::thread::spawn(move || {
        use url_open::UrlOpen;
        url::Url::parse(format!("{}://{}:{}/", protocol, addr, port).as_str())
            .unwrap()
            .open()
    });
}

#[cfg(feature = "open_browser")]
#[inline]
pub fn is_open_browser() -> bool {
    let mut args: Vec<String> = env::args().collect();

    if args.contains(&"-n".into()) || args.contains(&"--no-open".into()) {
        false
    } else {
        args.remove(0);

        if args.is_empty() {
            true
        } else {
            let bin_name = env::current_exe()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            eprintln!("Usage: {} [-n | --no-open]", bin_name);
            std::process::exit(1);
        }
    }
}

mod detect;
mod locales;

pub use detect::detect_locale;
pub use locales::Locales;

pub mod prelude {
    rust_i18n::i18n!(backend = super::Locales::new(), fallback = "en_US");
}

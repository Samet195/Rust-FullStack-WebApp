#[cfg(target_family = "unix")]
pub fn detect_locale() -> Option<String> {
    match std::env::var("LANG") {
        Ok(val) => Some(val.split('.').next()?.to_string()),
        Err(_) => None,
    }
}

#[cfg(target_family = "wasm")]
pub fn detect_locale() -> Option<String> {
    match js_sys::eval("navigator.language") {
        Ok(val) => val.as_string().map(|s| s.replace('-', "_").to_string()),
        Err(_) => None,
    }
}

#[cfg(target_family = "windows")]
pub fn detect_locale() -> Option<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use winapi::ctypes::c_int;
    use winapi::um::winnls::LCTYPE;
    use winapi::um::winnt::LCID;
    use winapi::um::winnt::WCHAR;

    let langid = unsafe { winapi::um::winnls::GetUserDefaultUILanguage() };

    let mut lang_buf: [WCHAR; 9] = [0; 9];
    const LOCALE_SISO639LANGNAME: LCTYPE = 0x59;
    let buf_size = unsafe {
        winapi::um::winnls::GetLocaleInfoW(
            langid as LCID,
            LOCALE_SISO639LANGNAME,
            lang_buf.as_mut_ptr(),
            lang_buf.len() as c_int,
        )
    } as usize;

    if buf_size > 0 {
        if let Some(lang) = OsString::from_wide(&lang_buf[..buf_size - 1])
            .into_string()
            .ok()
        {
            return Locale::from(&lang);
        }
    }

    None
}

// Extenal Crates Imports
use yew::Callback;

pub fn open_menu_cb<M: ToString, T>(menu: M) -> Callback<T> {
    let menu = menu.to_string();
    Callback::from(move |_| open_menu(menu.as_str()))
}

pub fn close_menu_cb<M: ToString, T>(menu: M) -> Callback<T> {
    let menu = menu.to_string();
    Callback::from(move |_| close_menu(menu.as_str()))
}

////////////////////////////////////////////////

pub fn open_menu<M: ToString>(menu: M) {
    let script = format!("openMenu('{}')", menu.to_string());
    let _ = js_sys::eval(script.as_str());
}

pub fn close_menu<M: ToString>(menu: M) {
    let script = format!("closeMenu('{}')", menu.to_string());
    let _ = js_sys::eval(script.as_str());
}

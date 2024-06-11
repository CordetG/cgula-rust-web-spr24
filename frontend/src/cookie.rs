//! `cookie.rs`
//!
//! This module contains a function for handling cookies in a web application context.
//!
//! The `acquire_cookie` function is used to retrieve a cookie named "test" from the client's browser.
//! If the cookie does not exist, the function sets it with a value of "123" and returns this value.
//! If the cookie does exist, the function simply returns its value.
//!
//! The cookies are managed using the `wasm_cookies` crate and the `cookies` module from the current crate.
//! The cookies are set to expire after a duration of 52 weeks.
//!
//! This module is particularly useful for managing user sessions or preferences that need to be stored in cookies.

use crate::*;

/// The function `acquire_cookie` retrieves a cookie named "test" and sets it to "123" if it
/// doesn't exist, returning the cookie value.
///
/// Returns:
///
/// The function `acquire_cookie()` returns a `String`. If a cookie with the name "test" is found, its
/// value is returned. If no cookie is found, a new cookie with the value "123" is set and "123" is
/// returned as the default value.
pub fn acquire_cookie() -> String {
    let cookie_options: wasm_cookies::CookieOptions = cookies::CookieOptions::default()
        .expires_after(core::time::Duration::from_secs(52 * 7 * 24 * 60 * 60));
    match cookies::get("test") {
        Some(Ok(cookie)) => {
            // log!("got cookie");
            return cookie;
        }
        Some(Err(_)) => {
            // log!(format!("cookie error: {}", e));
        }
        None => {
            // log!("did not find cookie");
        }
    }
    // log!("setting cookie");
    cookies::set("test", "123", &cookie_options);
    "123".to_string()
}

/// The function `render_cookie` generates HTML code to display a cookie message.
///
/// Arguments:
///
/// * `cookie`: A string representing the cookie that you want to render in an HTML paragraph element.
///
/// Returns:
///
/// The `render_cookie` function returns an HTML element `<div>` containing a `<p>` element with the
/// content of the `cookie` string passed as a parameter.
pub fn render_cookie(cookie: &str) -> Html {
    html! {
        <div>
            <p>{cookie}</p>
        </div>
    }
}

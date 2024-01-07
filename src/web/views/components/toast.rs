use std::fmt::Display;

use maud::Markup;

#[allow(dead_code)] //Silence compiler about unused levels
pub enum ToastLevel {
    None,
    Info,
    Success,
    Warning,
    Error,
}

pub fn toasts_container() -> Markup {
    maud::html! {
        // TODO: Use morphing for appending?
        div #toasts .toast hx-swap-oob="beforeend" {}
    }
}
pub fn toast(level: ToastLevel, msg: impl Display) -> Markup {
    let level = match level {
        ToastLevel::None => "",
        ToastLevel::Info => "alert-info",
        ToastLevel::Success => "alert-success",
        ToastLevel::Warning => "alert-warning",
        ToastLevel::Error => "alert-error",
    };

    maud::html! {
        // TODO: Use morphing for appending?
        div #toasts .toast hx-swap-oob="beforeend" {
            div .{"alert " (level)} _="init wait 5s then transition opacity to 0 then remove me" {
                span { (msg) }
            }
        }
    }
}

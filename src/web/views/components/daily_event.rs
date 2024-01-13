use maud::Markup;

use crate::daily;

use super::toast::{toast, ToastLevel};

pub fn to_html(event: daily::Event) -> Markup {
    match event {
        daily::Event::NewPersonJoined(p) => {
            maud::html! {
                div #participants hx-swap-oob="beforeend" {
                      p #(p.name()) { (p.name()) }
                }
                (toast(ToastLevel::Info, "NewPersonJoined"))
            }
        }
        daily::Event::RaisedHand => toast(ToastLevel::Info, "RaisedHand"),
        daily::Event::PersonLeft(p) => maud::html! {
            p #(p.name()) hx-swap-oob="delete" { (p.name()) }
            (toast(ToastLevel::Info, "PersonLeft"))
        },
    }
}

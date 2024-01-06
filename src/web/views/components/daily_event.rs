use maud::Markup;

use crate::daily;

use super::toast::{toast, ToastLevel};

pub fn to_html(event: daily::Event) -> Markup {
    let event_type = match event {
        daily::Event::NewPersonJoined => "NewPersonJoined",
        daily::Event::RaisedHand => "RaisedHand",
        daily::Event::PersonLeft => "PersonLeft",
    };

    toast(ToastLevel::Info, event_type)
}

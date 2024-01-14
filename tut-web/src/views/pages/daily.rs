use maud::Markup;
use tut::daily::{DailyId, DailyState};

use crate::views::{components::daily_state, layouts::root};

pub fn page(daily_id: DailyId, state: &DailyState) -> Markup {
    let body = maud::html! {
        header {
            nav ."navbar bg-base-100 shadow-xl px-4" {
                div ."navbar-start" {
                    a ."btn btn-ghost text-xl" role="button" href="/" { "Home" }
                }
            }
        }
        main ."container mx-auto m-8 rounded-box shadow-xl bg-base-200" {
            section ."p-8" {
                p { (daily_id) }
                (daily_state::html(daily_id, state))
            }
        }
    };

    root::layout(body)
}

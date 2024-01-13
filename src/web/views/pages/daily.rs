use maud::Markup;

use crate::{
    daily::{DailyId, DailyState},
    web::views::{
        components::{daily_state, toast},
        layouts::root,
    },
};

pub fn page(daily_id: DailyId, initial_state: &DailyState) -> Markup {
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
                div ."container mx-auto flex flex-col" hx-ext="ws" ws-connect={"/daily/" (daily_id) "/ws"} {
                    (daily_state::html(initial_state))
                    (toast::container())
                }
            }
        }
    };

    root::layout(body)
}

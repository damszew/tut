use maud::Markup;
use tut::daily::DailyId;

use crate::views::layouts::root;

pub fn page(daily_id: DailyId) -> Markup {
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
                form ."flex flex-col gap-4" method="post" action={"/daily/" (daily_id) "/join"} {
                    h1 { "Joining daily: " (daily_id) }
                    br;
                    div ."form-control w-full max-w-xs" {
                        label ."label" for="name"{
                            span ."label-text" { "Name" }
                        }
                        input #name ."input input-bordered w-full max-w-xs" name="name" type="text" {}
                    }
                    div ."divider" {}

                    // TODO: Add controls for to render as `disabled`
                    // TODO: Figure out how to remove daisyUI forcing transparent bg on submit btn
                    button ."btn btn-primary btn-outline" type="submit"{ "Lets go" }
                }
            }
        }
    };

    root::layout(body)
}

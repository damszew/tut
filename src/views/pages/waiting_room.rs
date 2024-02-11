use maud::Markup;
use tut_core::daily::{DailyId, DailyState};

use crate::views::{components::daily_state, layouts::root};

pub fn page(daily_id: DailyId, state: &DailyState) -> Markup {
    // TODO: pass it
    let url = format!("http://localhost:8000/daily/{daily_id}");
    let is_ready = false;
    let participants = vec![
        ("Perico", true),
        ("Rita", true),
        ("Jan", false),
        ("Anna", false),
    ];

    let body = maud::html! {
        header {
            nav ."navbar bg-base-100 shadow-xl px-4" {
                div ."navbar-start" {
                    a ."btn btn-ghost text-xl" role="button" href="/" { "Home" }
                }
            }
        }
        main ."container mx-auto m-8 max-w-md flex flex-col gap-4" {
            section ."p-8 rounded-box shadow-xl bg-base-200" {
                button
                ."btn btn-accent btn-outline btn-block"
                _ = { "on click"
                        "\nwriteText(\""(url)"\") on navigator.clipboard"
                        "\nput 'Copied!' into me"
                        "\nwait 1s"
                        "\nput \""(url)"\" into me"
                    }

            {
                { (url) }
            }
            }

            section ."p-8 rounded-box shadow-xl bg-base-200" {
                div ."flex flex-col rounded-box gap-4 p-4 bg-base-300" {
                    p ."text-lg" { "Participants" }
                    div #participants ."flex flex-col gap-2 max-w-fit px-4" {
                        @for participant in &participants {
                            div ."flex items-center rounded-badge bg-base-100"{
                                div  #(participant.0) .{"avatar placeholder flex " @if participant.1 {"online"}} {
                                    div ."bg-neutral text-neutral-content rounded-full w-12" {
                                        span {(participant.0.chars().take(2).collect::<String>())}
                                    }
                                }
                                p ."px-4" { (participant.0) }
                            }
                        }
                    }
                }
            }

            section ."p-8 rounded-box shadow-xl bg-base-200" {
                @if !is_ready {
                    button
                        ."btn btn-success btn-block"
                        hx-put={"/daily/" (daily_id) "?ready=true"}
                        // TODO: Swap toggle
                    {
                        "I'm so ready"
                    }
                } @else {
                    button
                        ."btn btn-error btn-block"
                        hx-put={"/daily/" (daily_id) "?ready=false"}
                        // TODO: Swap toggle
                    {
                        "Wait! I was not ready"
                    }
                }
            }
        }
    };

    root::layout(body)
}

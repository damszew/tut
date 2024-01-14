use maud::Markup;

use crate::views::layouts::root;

pub fn page() -> Markup {
    let body = maud::html! {
        header {
            nav ."navbar bg-base-100 shadow-xl px-4" {
                div ."navbar-start" {
                    a ."btn btn-ghost text-xl" role="button" href="/" { "Home" }
                }
                div ."navbar-end" {
                    a ."btn btn-primary" role="button" href="/daily/new" { "Start daily!" }
                }
            }
        }
        main ."container mx-auto p-4" {
            section {
                div ."hero min-h-screen bg-base-200" {
                    div ."hero-content text-center" {
                        div ."max-w-md" {
                            h1 ."text-5xl font-bold" { "Hello there" }
                            p ."py-6" { "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi." }
                            a ."btn btn-primary" role="button" href="/daily/new" { "Start daily" }
                        }
                    }
                }
            }
        }
        footer ."footer p-10 bg-base-200 text-base-content" {
            aside {
                // placeholder for logo
                svg .skeleton width="50" height="50" {}
                p { "Copyright © 2023 - All right reserved" }
            }
            nav {
                header ."footer-title" { "Company" }
                a ."link link-hover" { "Contact" }
            }
        }



    };

    root::layout(body)
}

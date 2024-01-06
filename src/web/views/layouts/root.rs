use maud::Markup;

pub fn layout(inner: Markup) -> Markup {
    maud::html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";

                script src="https://unpkg.com/htmx.org@1.9.9" {}
                script src="https://unpkg.com/hyperscript.org@0.9.12" {}
                script src="https://unpkg.com/htmx.org/dist/ext/ws.js" {}

                link
                    href="https://cdn.jsdelivr.net/npm/daisyui@4.4.19/dist/full.min.css"
                    rel="stylesheet"
                    type_="text/css" {}
                script src="https://cdn.tailwindcss.com" {}

            }
            body ."bg-base-300" data-theme="dracula" {
                (inner)
            }
        }

    }
}

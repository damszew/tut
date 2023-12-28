import gleam/string_builder.{type StringBuilder}
import lustre/element/html
import lustre/element
import lustre/attribute

pub fn page(daily_id: String) -> StringBuilder {
  let doctype = string_builder.from_string("<!doctype html>")

  let content =
    html.html([attribute.attribute("lang", "en")], [
      html.head([], [
        html.meta([attribute.attribute("charset", "UTF-8")]),
        html.meta([
          attribute.name("viewport"),
          attribute.attribute("content", "width=device-width, initial-scale=1.0",
          ),
        ]),
        html.link([
          attribute.href(
            "https://cdn.jsdelivr.net/npm/daisyui@4.4.19/dist/full.min.css",
          ),
          attribute.rel("stylesheet"),
          attribute.type_("text/css"),
        ]),
        html.script([attribute.src("https://cdn.tailwindcss.com")], ""),
        html.script([attribute.src("https://unpkg.com/htmx.org@1.9.9")], ""),
        html.script(
          [attribute.src("https://unpkg.com/htmx.org/dist/ext/ws.js")],
          "",
        ),
      ]),
      html.body(
        [
          attribute.attribute("data-theme", "dracula"),
          attribute.class("bg-base-300 min-h-screen"),
        ],
        [
          html.header([], [
            html.nav([attribute.class("navbar bg-base-100 shadow-xl px-4")], [
              html.div([attribute.class("navbar-start")], [
                html.a(
                  [
                    attribute.attribute("role", "button"),
                    attribute.class("btn btn-ghost text-xl"),
                    attribute.href("/"),
                  ],
                  [element.text("Home")],
                ),
              ]),
            ]),
          ]),
          html.main(
            [
              attribute.class(
                "container mx-auto m-8 rounded-box shadow-xl bg-base-200",
              ),
            ],
            [
              html.section([attribute.class("p-8")], [
                html.p([], [element.text(daily_id)]),
                html.div(
                  [
                    attribute.attribute("hx-ext", "ws"),
                    attribute.attribute("ws-connect", "/ws/" <> daily_id),
                    attribute.class("container mx-auto flex flex-col"),
                  ],
                  [],
                ),
              ]),
            ],
          ),
        ],
      ),
    ])
    |> element.to_string_builder()

  string_builder.new()
  |> string_builder.append_builder(doctype)
  |> string_builder.append_builder(content)
}

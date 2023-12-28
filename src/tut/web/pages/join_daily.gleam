import gleam/string_builder.{type StringBuilder}
import lustre/element/html
import lustre/element
import lustre/attribute
import tut/web/pages/layouts/root

pub fn page(daily_id: String) -> StringBuilder {
  root.layout([
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
  ])
  |> element.to_string_builder()
}

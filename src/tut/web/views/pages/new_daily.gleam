import gleam/string_builder
import lustre/element/html
import lustre/element
import lustre/attribute
import tut/web/views/layouts/root

pub fn page() -> string_builder.StringBuilder {
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
          html.form(
            [
              attribute.attribute("method", "post"),
              attribute.attribute("action", "/daily"),
              attribute.class("flex flex-col gap-4"),
            ],
            [
              html.div([attribute.class("form-control w-full max-w-xs")], [
                html.label([attribute.class("label"), attribute.for("name")], [
                  html.span([attribute.class("label-text")], [
                    element.text("Name"),
                  ]),
                ]),
                html.input([
                  attribute.id("name"),
                  attribute.name("name"),
                  attribute.type_("text"),
                  attribute.class("input input-bordered w-full max-w-xs"),
                ]),
              ]),
              html.div([attribute.class("divider")], []),
              // TODO: Add controls for to render as `disabled`
              html.button(
                [
                  // TODO: Figureout how to remove daisyUI forcing transparent bg on submit btn
                  attribute.type_("submit"),
                  attribute.class("btn btn-primary btn-outline"),
                ],
                [element.text("Lets go")],
              ),
            ],
          ),
        ]),
      ],
    ),
  ])
  |> element.to_string_builder()
}

import gleam/string_builder.{type StringBuilder}
import lustre/element/html
import lustre/element
import lustre/attribute
import tut/web/views/layouts/root

pub fn page() -> StringBuilder {
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
        html.div([attribute.class("navbar-end")], [
          html.a(
            [
              attribute.attribute("role", "button"),
              attribute.class("btn btn-primary"),
              attribute.href("/daily/new"),
            ],
            [element.text("Start daily!")],
          ),
        ]),
      ]),
    ]),
    html.main([attribute.class("container mx-auto p-4")], [
      html.section([], [
        html.div([attribute.class("hero min-h-screen bg-base-200")], [
          html.div([attribute.class("hero-content text-center")], [
            html.div([attribute.class("max-w-md")], [
              html.h1([attribute.class("text-5xl font-bold")], [
                element.text("Hello there"),
              ]),
              html.p([attribute.class("py-6")], [
                element.text(
                  "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.",
                ),
              ]),
              html.a(
                [
                  attribute.attribute("role", "button"),
                  attribute.class("btn btn-primary"),
                  attribute.href("/daily/new"),
                ],
                [element.text("Start daily")],
              ),
            ]),
          ]),
        ]),
      ]),
    ]),
    html.footer([attribute.class("footer p-10 bg-base-200 text-base-content")], [
      html.aside([], [
        // placeholder for logo
        html.svg(
          [
            attribute.class("skeleton"),
            attribute.width(50),
            attribute.height(50),
          ],
          [],
        ),
        html.p([], [element.text("Copyright © 2023 - All right reserved")]),
      ]),
      html.nav([], [
        html.header([attribute.class("footer-title")], [element.text("Company")]),
        html.a([attribute.class("link link-hover")], [element.text("Contact")]),
      ]),
    ]),
  ])
  |> element.to_string_builder()
}

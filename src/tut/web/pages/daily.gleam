import gleam/string_builder
import wisp.{type Request, type Response}
import lustre/element/html
import lustre/element
import lustre/attribute
import tut/daily_router

pub fn page_new(_: Request) -> Response {
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
                html.form(
                  [
                    attribute.attribute("method", "post"),
                    attribute.attribute("action", "/daily"),
                    attribute.class("flex flex-col gap-4"),
                  ],
                  [
                    html.div([attribute.class("form-control w-full max-w-xs")], [
                      html.label(
                        [attribute.class("label"), attribute.for("name")],
                        [
                          html.span([attribute.class("label-text")], [
                            element.text("Name"),
                          ]),
                        ],
                      ),
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
        ],
      ),
    ])
    |> element.to_string_builder()

  let body =
    string_builder.new()
    |> string_builder.append_builder(doctype)
    |> string_builder.append_builder(content)

  // Return a 200 OK response with the body and a HTML content type.
  wisp.html_response(body, 200)
}

pub fn create(req: Request, daily_router: daily_router.DailyRouter) -> Response {
  use _formdata <- wisp.require_form(req)

  let token = daily_router.create(daily_router)

  wisp.redirect(to: "daily/" <> token)
}

pub fn join(_: Request, daily_id: String) -> Response {
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

  let body =
    string_builder.new()
    |> string_builder.append_builder(doctype)
    |> string_builder.append_builder(content)

  // Return a 200 OK response with the body and a HTML content type.
  wisp.html_response(body, 200)
}

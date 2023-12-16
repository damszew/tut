import wisp.{type Request, type Response}
import gleam/string_builder
import lustre/element/html
import lustre/element
import lustre/attribute
import tut/web

pub fn handle_request(req: Request) -> Response {
  // Apply the middleware stack for this request/response.
  use _req <- web.middleware(req)

  case wisp.path_segments(req) {
    // This matches `/`.
    [] -> home_page(req)

    _ -> wisp.not_found()
  }
}

fn home_page(_: Request) -> Response {
  let doctype = string_builder.from_string("<!doctype html>")

  let content =
    html.html(
      [attribute.attribute("lang", "en")],
      [
        html.head(
          [],
          [
            html.meta([attribute.attribute("charset", "UTF-8")]),
            html.meta([
              attribute.name("viewport"),
              attribute.attribute(
                "content",
                "width=device-width, initial-scale=1.0",
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
          ],
        ),
        html.body(
          [
            attribute.attribute("data-theme", "dracula"),
            attribute.class("bg-base-300"),
          ],
          [
            html.header(
              [],
              [
                html.nav(
                  [attribute.class("navbar bg-base-100 shadow-xl px-4")],
                  [
                    html.div(
                      [attribute.class("navbar-start")],
                      [
                        html.a(
                          [
                            attribute.attribute("role", "button"),
                            attribute.class("btn btn-ghost text-xl"),
                            attribute.href("/"),
                          ],
                          [element.text("Home")],
                        ),
                      ],
                    ),
                    html.div(
                      [attribute.class("navbar-end")],
                      [
                        html.a(
                          [
                            attribute.attribute("role", "button"),
                            attribute.class("btn btn-primary"),
                            attribute.href("/daily/new"),
                          ],
                          [element.text("Start daily!")],
                        ),
                      ],
                    ),
                  ],
                ),
              ],
            ),
            html.main(
              [attribute.class("container mx-auto p-4")],
              [
                html.section(
                  [],
                  [
                    html.div(
                      [attribute.class("hero min-h-screen bg-base-200")],
                      [
                        html.div(
                          [attribute.class("hero-content text-center")],
                          [
                            html.div(
                              [attribute.class("max-w-md")],
                              [
                                html.h1(
                                  [attribute.class("text-5xl font-bold")],
                                  [element.text("Hello there")],
                                ),
                                html.p(
                                  [attribute.class("py-6")],
                                  [
                                    element.text(
                                      "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.",
                                    ),
                                  ],
                                ),
                                html.a(
                                  [
                                    attribute.attribute("role", "button"),
                                    attribute.class("btn btn-primary"),
                                    attribute.href("/daily/new"),
                                  ],
                                  [element.text("Start daily")],
                                ),
                              ],
                            ),
                          ],
                        ),
                      ],
                    ),
                  ],
                ),
              ],
            ),
            html.footer(
              [attribute.class("footer p-10 bg-base-200 text-base-content")],
              [
                html.aside(
                  [],
                  [
                    // placeholder for logo
                    html.svg(
                      [
                        attribute.class("skeleton"),
                        attribute.width(50),
                        attribute.height(50),
                      ],
                      [],
                    ),
                    html.p(
                      [],
                      [element.text("Copyright © 2023 - All right reserved")],
                    ),
                  ],
                ),
                html.nav(
                  [],
                  [
                    html.header(
                      [attribute.class("footer-title")],
                      [element.text("Company")],
                    ),
                    html.a(
                      [attribute.class("link link-hover")],
                      [element.text("Contact")],
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
      ],
    )
    |> element.to_string_builder()

  let body =
    string_builder.new()
    |> string_builder.append_builder(doctype)
    |> string_builder.append_builder(content)

  // Return a 200 OK response with the body and a HTML content type.
  wisp.html_response(body, 200)
}

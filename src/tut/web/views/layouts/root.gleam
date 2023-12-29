import lustre/element/html
import lustre/element.{type Element}
import lustre/attribute

pub fn layout(inner: List(Element(a))) -> Element(a) {
  html.html([attribute.attribute("lang", "en")], [
    html.head([], [
      html.meta([attribute.attribute("charset", "UTF-8")]),
      html.meta([
        attribute.name("viewport"),
        attribute.attribute("content", "width=device-width, initial-scale=1.0"),
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
        attribute.class("bg-base-300"),
      ],
      inner,
    ),
  ])
}

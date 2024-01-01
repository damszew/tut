import lustre/element/html
import lustre/element
import lustre/attribute

pub fn toasts_container() -> element.Element(a) {
  html.div(
    [
      attribute.id("toasts"),
      attribute.class("toast"),
      // TODO: Use morphing for appending?
      attribute.attribute("hx-swap-oob", "beforeend"),
    ],
    [],
  )
}

pub type AlertLevel {
  None
  Info
  Success
  Warning
  Error
}

pub fn toast(level: AlertLevel, msg: String) -> element.Element(a) {
  let alert_level = case level {
    None -> ""
    Info -> "alert-info"
    Success -> "alert-success"
    Warning -> "alert-warning"
    Error -> "alert-error"
  }

  html.div(
    [
      attribute.id("toasts"),
      attribute.class("toast"),
      // TODO: Use morphing for appending?
      attribute.attribute("hx-swap-oob", "beforeend"),
    ],
    [
      html.div(
        [
          attribute.class("alert " <> alert_level),
          attribute.attribute(
            "_",
            "init wait 5s then transition opacity to 0 then remove me",
          ),
        ],
        [html.span([], [element.text(msg)])],
      ),
    ],
  )
}

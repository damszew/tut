import gleam/string_builder.{type StringBuilder}
import lustre/element/html
import lustre/element
import lustre/attribute
import tut/daily

pub fn to_html(event: daily.Event) -> StringBuilder {
  let event_type = case event {
    daily.NewPersonJoined -> "NewPersonJoined"
    daily.RaisedHand -> "RaisedHand"
    daily.PersonLeft -> "PersonLeft"
  }

  html.div(
    [
      attribute.id("event-container"),
      attribute.class("container mx-auto m-8 rounded-box shadow-xl bg-primary"),
    ],
    [
      html.section([attribute.class("p-8")], [
        html.p([], [element.text(event_type)]),
      ]),
    ],
  )
  |> element.to_string_builder()
}

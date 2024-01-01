import gleam/string_builder.{type StringBuilder}
import lustre/element
import tut/daily
import tut/web/views/components/toast

pub fn to_html(event: daily.Event) -> StringBuilder {
  let event_type = case event {
    daily.NewPersonJoined -> "NewPersonJoined"
    daily.RaisedHand -> "RaisedHand"
    daily.PersonLeft -> "PersonLeft"
  }

  toast.toast(toast.Info, event_type)
  |> element.to_string_builder()
}

import wisp
import tut/daily_router
import tut/daily
import tut/web/views/pages/new_daily
import tut/web/views/pages/join_daily
import tut/web/views/components/daily_event
import gleam/http/request.{type Request}
import gleam/http/response.{type Response}
import gleam/dynamic
import gleam/erlang/process
import gleam/otp/actor
import gleam/option
import gleam/result
import gleam/bit_array
import gleam/string_builder
import gleam/json
import mist.{
  type Connection, type ResponseData, type WebsocketConnection,
  type WebsocketMessage,
}

pub fn page_new(_: wisp.Request) -> wisp.Response {
  let body = new_daily.page()

  wisp.html_response(body, 200)
}

pub fn create(
  req: wisp.Request,
  daily_router: daily_router.DailyRouter,
) -> wisp.Response {
  use _formdata <- wisp.require_form(req)

  let token = daily_router.create(daily_router)

  wisp.redirect(to: "daily/" <> token)
}

pub fn join(
  _: wisp.Request,
  daily_id: String,
  daily_router: daily_router.DailyRouter,
) -> wisp.Response {
  case daily_router.daily_exists(daily_router, daily_id) {
    True -> {
      let body = join_daily.page(daily_id)
      wisp.html_response(body, 200)
    }
    // TODO: Display some error about what happened
    False -> wisp.redirect(to: "/")
  }
}

pub fn daily_websocket(
  req: Request(Connection),
  daily_id: String,
  daily_router: daily_router.DailyRouter,
) -> Response(ResponseData) {
  mist.websocket(
    request: req,
    on_init: init_websocket(_, daily_id, daily_router),
    on_close: fn(_) { Nil },
    handler: handle_ws_message,
  )
}

fn init_websocket(
  _conn: WebsocketConnection,
  daily_id: String,
  daily_router: daily_router.DailyRouter,
) -> #(daily.Daily, option.Option(process.Selector(WsMessage))) {
  let ws_subject = process.new_subject()
  let ws_selector =
    process.new_selector()
    |> process.selecting(ws_subject, Event)

  let daily =
    daily_router
    |> daily_router.join(daily_id, ws_subject)

  #(daily, option.Some(ws_selector))
}

pub type WsMessage {
  Event(daily.Event)
}

fn handle_ws_message(
  daily: daily.Daily,
  conn: WebsocketConnection,
  message: WebsocketMessage(WsMessage),
) {
  case message {
    mist.Text(bits) -> {
      let msg =
        bits
        |> bit_array.to_string()
        |> result.unwrap("")
        |> from_json()
        |> to_event()

      daily
      |> daily.send(msg)

      actor.continue(daily)
    }

    mist.Custom(Event(event)) -> {
      let text =
        event
        |> daily_event.to_html()
        |> string_builder.to_string()

      let assert Ok(_) = mist.send_text_frame(conn, <<text:utf8>>)
      actor.continue(daily)
    }

    mist.Binary(_) -> {
      actor.continue(daily)
    }

    mist.Closed | mist.Shutdown -> actor.Stop(process.Normal)
  }
}

type DailyEventDto {
  DailyEventDto(event: String, headers: dynamic.Dynamic)
}

fn from_json(json_string: String) -> DailyEventDto {
  let raw_decoder =
    dynamic.decode2(
      DailyEventDto,
      dynamic.field("event", of: dynamic.string),
      dynamic.field("HEADERS", of: dynamic.dynamic),
    )

  let assert Ok(result) = json.decode(from: json_string, using: raw_decoder)

  result
}

fn to_event(msg: DailyEventDto) -> daily.Event {
  case msg.event {
    "NewPersonJoined" -> daily.NewPersonJoined
    "RaisedHand" -> daily.RaisedHand
    "PersonLeft" -> daily.PersonLeft
    _ -> panic
  }
}

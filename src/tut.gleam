import wisp
import mist.{
  type Connection, type ResponseData, type WebsocketConnection,
  type WebsocketMessage,
}
import gleam/http/request.{type Request}
import gleam/http/response.{type Response}
import gleam/erlang/process
import gleam/otp/actor
import gleam/bit_array
import gleam/option
import gleam/result
import tut/web/router
import tut/daily_router
import tut/daily

pub fn main() {
  wisp.configure_logger()

  // TODO: Pass key from env
  // Here we generate a secret key, but in a real application you would want to
  // load this from somewhere so that it is not regenerated on every restart.
  let secret_key_base = wisp.random_string(64)

  let assert Ok(daily_router) = daily_router.new()

  let context = router.Context(db: daily_router)

  let wisp_handler =
    router.handle_request(_, context)
    |> wisp.mist_handler(secret_key_base)

  // WA: Until https://github.com/gleam-wisp/wisp/issues/10
  fn(req: Request(Connection)) -> Response(ResponseData) {
    case request.path_segments(req) {
      ["ws", daily_id] -> {
        mist.websocket(
          request: req,
          on_init: fn(_conn) {
            let ws_subject = process.new_subject()
            let ws_selector =
              process.new_selector()
              |> process.selecting(ws_subject, Event)

            let daily =
              daily_router
              |> daily_router.join(daily_id, ws_subject)

            #(daily, option.Some(ws_selector))
          },
          on_close: fn(_) { Nil },
          handler: handle_ws_message,
        )
      }

      _ -> wisp_handler(req)
    }
  }
  |> mist.new
  |> mist.port(8000)
  |> mist.start_http

  process.sleep_forever()
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
    // Read
    mist.Text(bits) -> {
      let msg =
        bits
        |> bit_array.to_string
        |> result.unwrap("")
        |> from_json()
        |> to_event()

      daily
      |> daily.send(msg)

      actor.continue(daily)
    }

    // Write
    mist.Custom(Event(event)) -> {
      let text =
        event
        |> to_string()

      let assert Ok(_) = mist.send_text_frame(conn, <<text:utf8>>)
      actor.continue(daily)
    }

    // Ignore
    mist.Binary(_) -> {
      actor.continue(daily)
    }

    // Clean up
    mist.Closed | mist.Shutdown -> actor.Stop(process.Normal)
  }
}

import gleam/dynamic
import gleam/json

type Msg {
  Msg(event: String, headers: dynamic.Dynamic)
}

pub fn from_json(json_string: String) -> String {
  let raw_decoder =
    dynamic.decode2(
      Msg,
      dynamic.field("event", of: dynamic.string),
      dynamic.field("HEADERS", of: dynamic.dynamic),
    )

  let assert Ok(result) = json.decode(from: json_string, using: raw_decoder)

  result.event
}

pub fn to_event(msg: String) -> daily.Event {
  case msg {
    "NewPersonJoined" -> daily.NewPersonJoined
    "RaisedHand" -> daily.RaisedHand
    "PersonLeft" -> daily.PersonLeft
    _ -> todo
  }
}

pub fn to_string(msg: daily.Event) -> String {
  case msg {
    daily.NewPersonJoined -> "NewPersonJoined"
    daily.RaisedHand -> "RaisedHand"
    daily.PersonLeft -> "PersonLeft"
  }
}

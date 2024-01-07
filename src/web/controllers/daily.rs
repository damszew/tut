use std::collections::HashMap;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, WebSocketUpgrade,
    },
    response::{IntoResponse, Redirect, Response},
    Form,
};
use futures::{
    sink::SinkExt,
    stream::{self, StreamExt},
    Stream,
};

use crate::web::views::{
    components::daily_event,
    pages::{daily, new_daily},
};

pub async fn create_form() -> maud::Markup {
    new_daily::page()
}

pub async fn create(Form(_): Form<HashMap<String, String>>) -> Redirect {
    let token = "nana"; // TODO: create daily

    Redirect::to(&format!("daily/{token}"))
}

pub async fn room(Path(daily_id): Path<String>) -> Response {
    // TODO: Check if daily exists
    match true {
        true => daily::page(daily_id).into_response(),

        // TODO: Display error about what happened
        false => Redirect::to("/").into_response(),
    }
}

pub async fn websocket(ws: WebSocketUpgrade, Path(daily_id): Path<String>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, stream::pending()))
}

async fn handle_socket(
    socket: WebSocket,
    mut daily_events: impl Stream<Item = crate::daily::Event> + Unpin + Send + 'static,
) {
    tracing::info!("Opened ws");
    let (mut sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => break,

                Message::Text(_) => {
                    todo!("Receive msg from WS and pass it to daily");
                }
                _ => (),
            }
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Some(event) = daily_events.next().await {
            let text = daily_event::to_html(event).into_string();

            sender
                .send(Message::Text(text))
                .await
                .expect("Handle sender fail case");
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
}

// pub type WsMessage {
//   Event(daily.Event)
// }

// fn handle_ws_message(
//   daily: daily.Daily,
//   conn: WebsocketConnection,
//   message: WebsocketMessage(WsMessage),
// ) {
//   case message {
//     mist.Text(bits) -> {
//       let msg =
//         bits
//         |> bit_array.to_string()
//         |> result.unwrap("")
//         |> from_json()
//         |> to_event()

//       daily
//       |> daily.send(msg)

//       actor.continue(daily)
//     }

//     mist.Custom(Event(event)) -> {
//       let text =
//         event
//         |> daily_event.to_html()
//         |> string_builder.to_string()

//       let assert Ok(_) = mist.send_text_frame(conn, <<text:utf8>>)
//       actor.continue(daily)
//     }

//     mist.Binary(_) -> {
//       actor.continue(daily)
//     }

//     mist.Closed | mist.Shutdown -> actor.Stop(process.Normal)
//   }
// }

// type DailyEventDto {
//   DailyEventDto(event: String, headers: dynamic.Dynamic)
// }

// fn from_json(json_string: String) -> DailyEventDto {
//   let raw_decoder =
//     dynamic.decode2(
//       DailyEventDto,
//       dynamic.field("event", of: dynamic.string),
//       dynamic.field("HEADERS", of: dynamic.dynamic),
//     )

//   let assert Ok(result) = json.decode(from: json_string, using: raw_decoder)

//   result
// }

// fn to_event(msg: DailyEventDto) -> daily.Event {
//   case msg.event {
//     "NewPersonJoined" -> daily.NewPersonJoined
//     "RaisedHand" -> daily.RaisedHand
//     "PersonLeft" -> daily.PersonLeft
//     _ -> panic
//   }
// }

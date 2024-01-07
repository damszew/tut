use std::collections::HashMap;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::{IntoResponse, Redirect, Response},
    Form,
};
use futures::{sink::SinkExt, stream::StreamExt};

use crate::{
    daily::Daily,
    web::{
        router::AppState,
        views::{
            components::daily_event,
            pages::{daily, new_daily},
        },
    },
};

pub async fn create_form() -> maud::Markup {
    new_daily::page()
}

pub async fn create(
    State(app_state): State<AppState>,
    Form(_): Form<HashMap<String, String>>,
) -> Redirect {
    // TODO: Here should happen conversion from id to url-friendly id token
    let token = app_state.daily_router.create_daily().await;

    Redirect::to(&format!("daily/{token}"))
}

pub async fn room(Path(daily_id): Path<String>, State(app_state): State<AppState>) -> Response {
    let exists = app_state.daily_router.daily_exists(&daily_id).await;

    match exists {
        true => daily::page(daily_id).into_response(),

        // TODO: Display error about what happened
        false => Redirect::to("/").into_response(),
    }
}

pub async fn websocket(
    ws: WebSocketUpgrade,
    Path(daily_id): Path<String>,
    State(app_state): State<AppState>,
) -> Response {
    let daily = app_state.daily_router.join(&daily_id, ()).await;

    ws.on_upgrade(move |socket| handle_socket(socket, daily))
}

#[derive(Debug, serde::Deserialize)]
struct WsMsg {
    event: crate::daily::Event,

    #[serde(rename = "HEADERS")]
    headers: HashMap<String, String>,
}

async fn handle_socket(socket: WebSocket, daily: Daily) {
    tracing::info!("Opened ws");
    let (mut sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn({
        let daily = daily.clone();
        async move {
            while let Some(Ok(msg)) = receiver.next().await {
                match msg {
                    Message::Close(_) => break,

                    Message::Text(text) => {
                        let msg = serde_json::from_str::<WsMsg>(&text)
                            .expect("Handle serialization error in websocket");
                        daily.send(msg.event).await;
                    }
                    _ => (),
                }
            }
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = daily.recv().await {
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

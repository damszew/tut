use tut_core::{
    daily::{DailyId, DailyState, Event},
    participant::Participant,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    router::AppState,
    views::pages::{
        join_daily, new_daily,
        waiting_room::{self, WaitingRoomView},
    },
};

pub async fn create_form() -> maud::Markup {
    new_daily::page()
}

#[derive(Debug, serde::Deserialize)]
pub struct JoinDailyReq {
    name: String,
}

pub async fn create(
    State(app_state): State<AppState>,
    jar: CookieJar,
    Form(req): Form<JoinDailyReq>,
) -> (CookieJar, Redirect) {
    // TODO: Here should happen conversion from id to url-friendly id token
    let daily_id = app_state.daily_router.create_daily().await;

    let daily = app_state.daily_router.get(&daily_id).await.unwrap();

    let me = Participant::new(req.name);
    let jar = jar.add(Cookie::new(
        daily_id.to_string(),
        me.id.to_string().to_owned(),
    ));

    daily.event(Event::Join(me)).await.unwrap();

    (jar, Redirect::to(&format!("/daily/{daily_id}")))
}

pub async fn join(
    Path(daily_id): Path<DailyId>,
    State(app_state): State<AppState>,
    jar: CookieJar,
    Form(req): Form<JoinDailyReq>,
) -> (CookieJar, Redirect) {
    let daily = app_state.daily_router.get(&daily_id).await.unwrap();

    let me = Participant::new(req.name);
    let jar = jar.add(Cookie::new(
        daily_id.to_string(),
        me.id.to_string().to_owned(),
    ));

    daily.event(Event::Join(me)).await.unwrap();

    (jar, Redirect::to(&format!("/daily/{daily_id}")))
}

// TODO: Determine if participant is connected

pub async fn room(
    Path(daily_id): Path<DailyId>,
    jar: CookieJar,
    State(app_state): State<AppState>,
) -> Response {
    let daily = app_state.daily_router.get(&daily_id).await;
    let cookie = jar.get(&daily_id.to_string());

    match (daily, cookie) {
        (Some(daily), Some(cookie)) => {
            let daily_state = daily.state().await;
            let my_id = cookie.value().into();

            match daily_state {
                DailyState::WaitingRoom(room) => {
                    let waiting_room = WaitingRoomView {
                        url: "http://localhost:8000".into(), // TODO: Extract from req
                        daily_id,
                        me: my_id,
                        participants: room.participants,
                        ready_participants: room.ready_participants,
                    };
                    waiting_room::page(&waiting_room).into_response()
                }

                _ => todo!(),
            }
        }
        (Some(_), None) => join_daily::page(daily_id).into_response(),

        // TODO: Display error about what happened
        (None, _) => Redirect::to("/").into_response(),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ReadyDailyReq {
    ready: bool,
}

pub async fn ready(
    Path(daily_id): Path<DailyId>,
    State(app_state): State<AppState>,
    jar: CookieJar,
    Form(req): Form<ReadyDailyReq>,
) -> StatusCode {
    let daily = app_state.daily_router.get(&daily_id).await.unwrap();

    let cookie = jar.get(&daily_id.to_string()).unwrap();
    let participant_id = cookie.value().into();

    if req.ready {
        daily.event(Event::Ready(participant_id)).await.unwrap();
    }

    StatusCode::OK
}

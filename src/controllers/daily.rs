use tut_core::{daily::DailyId, participant::ParticipantId, waiting_room::WaitingRoom};

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    router::AppState,
    views::pages::{join_daily, new_daily, waiting_room},
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

    let me = ParticipantId::random();
    let jar = jar.add(Cookie::new(daily_id.to_string(), me.to_string().to_owned()));

    daily.join(me, req.name).await;

    (jar, Redirect::to(&format!("/daily/{daily_id}")))
}

pub async fn join(
    Path(daily_id): Path<DailyId>,
    State(app_state): State<AppState>,
    jar: CookieJar,
    Form(req): Form<JoinDailyReq>,
) -> (CookieJar, Redirect) {
    let daily = app_state.daily_router.get(&daily_id).await.unwrap();

    let me = ParticipantId::random();
    let jar = jar.add(Cookie::new(daily_id.to_string(), me.to_string().to_owned()));

    daily.join(me, req.name).await;

    (jar, Redirect::to(&format!("/daily/{daily_id}")))
}

pub async fn next_step(
    Path(daily_id): Path<DailyId>,
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> Response {
    let daily = app_state.daily_router.get(&daily_id).await;
    let cookie = jar.get(&daily_id.to_string());

    match (daily, cookie) {
        (Some(daily), Some(cookie)) => {
            let me = cookie.value().into();
            daily.ready_for_next_step(me).await;
            ().into_response()
        }

        // TODO: Display error about what happened
        (_, _) => Redirect::to("/").into_response(),
    }
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
        (Some(daily), Some(_)) => {
            let daily_state = daily.state().await;
            let waiting_room = WaitingRoom {
                url: "http://localhost:8000".into(), // TODO: Extract from req
                daily_id,
                am_i_ready: false, // TODO
                participants: daily_state.participants.into_values().collect(),
            };
            waiting_room::page(&waiting_room).into_response()
        }
        (Some(_), None) => join_daily::page(daily_id).into_response(),

        // TODO: Display error about what happened
        (None, _) => Redirect::to("/").into_response(),
    }
}

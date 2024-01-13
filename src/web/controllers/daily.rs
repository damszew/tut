use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    daily::{DailyId, Participant},
    web::{
        router::AppState,
        views::pages::{daily, join_daily, new_daily},
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

    Redirect::to(&format!("/daily/{token}"))
}

pub async fn join(
    Path(daily_id): Path<DailyId>,
    State(app_state): State<AppState>,
    jar: CookieJar,
    Form(_): Form<HashMap<String, String>>,
) -> (CookieJar, Redirect) {
    let daily = app_state.daily_router.get(&daily_id).await.unwrap();

    let me = Participant::default();
    let jar = jar.add(Cookie::new(daily_id.to_string(), me.name().to_owned()));

    daily.join(me).await;

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
        (Some(daily), Some(_)) => {
            let daily_state = daily.state().await;
            daily::page(daily_id, &daily_state).into_response()
        }
        (Some(_), None) => join_daily::page(daily_id).into_response(),

        // TODO: Display error about what happened
        (None, _) => Redirect::to("/").into_response(),
    }
}

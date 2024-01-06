use std::collections::HashMap;

use axum::{
    extract::Path,
    response::{IntoResponse, Redirect, Response},
    Form,
};

use crate::web::views::pages::{daily, new_daily};

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

use maud::Markup;

use crate::daily::{DailyId, DailyState};

pub fn html(daily_id: DailyId, state: &DailyState) -> Markup {
    maud::html! {
        div #daily-state hx-get={"/daily/" (daily_id)} hx-select="#daily-state" hx-trigger="every 2s"{
            br;
            h2 { "Participants" }
            div #participants {
                @for participant in &state.participants {
                    p #(participant.name()) { (participant.name()) }
                }
            }
        }
    }
}

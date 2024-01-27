use maud::Markup;
use tut::daily::{DailyId, DailyState};

pub fn html(daily_id: DailyId, state: &DailyState) -> Markup {
    let current_step = match state.step {
        tut::daily::DailyStep::Waiting => "Waiting",
        tut::daily::DailyStep::Started => "Started",
        tut::daily::DailyStep::Finished => "Finished",
    };

    maud::html! {
        div #daily-state hx-get={"/daily/" (daily_id)} hx-select="#daily-state" hx-trigger="every 2s"{
            br;
            h2 { "Participants" }
            div #participants {
                @for participant in &state.participants {
                    p #(participant.0) { (participant.0) }
                }
            }
            br;
            p { "Current step: " (current_step) }
            br;
            button
                ."btn btn-primary"
                hx-put={"/daily/" (daily_id) "/next_step"}
                hx-swap="none"
            {
                "Ready"
            }
        }
    }
}

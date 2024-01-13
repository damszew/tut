use maud::Markup;

use crate::daily::DailyState;

pub fn html(state: &DailyState) -> Markup {
    maud::html! {
        h2 { "Participants" }
        div #participants {
            @for participant in &state.participants {
                p #(participant.name()) { (participant.name()) }
            }
        }
    }
}

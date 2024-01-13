use maud::Markup;

use crate::daily::DailyState;

pub fn html(state: &DailyState) -> Markup {
    maud::html! {
        div #daily-state hx-swap-oob="beforeend" {
            h2 { "Participants" }
            div #participants {
                @for participant in &state.participants {
                    p #(participant.name()) { (participant.name()) }
                }
            }
        }
    }
}

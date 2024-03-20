use maud::Markup;
use tut_core::{daily::DailyId, participant::ParticipantId, waiting_room::WaitingRoom};

use crate::views::layouts::root;

pub struct WaitingRoomView {
    pub url: String,
    pub daily_id: DailyId,
    pub me: ParticipantId,
    pub room: WaitingRoom,
}

impl WaitingRoomView {
    pub fn page(&self) -> Markup {
        let url = format!("{}/daily/{}", self.url, self.daily_id);
        let is_ready = |id| self.room.ready_participants.contains(&id);

        let body = maud::html! {
            header {
                nav ."navbar bg-base-100 shadow-xl px-4" {
                    div ."navbar-start" {
                        a ."btn btn-ghost text-xl" role="button" href="/" { "Home" }
                    }
                }
            }
            main ."container mx-auto m-8 max-w-md flex flex-col gap-4" {
                section ."p-8 rounded-box shadow-xl bg-base-200" {
                    button
                        ."btn btn-accent btn-outline btn-block"
                        _ = { "on click"
                                "\nwriteText(\""(url)"\") on navigator.clipboard"
                                "\nput 'Copied!' into me"
                                "\nwait 1s"
                                "\nput \""(url)"\" into me"
                            }
                    {
                        (url)
                    }
                }

                section ."p-8 rounded-box shadow-xl bg-base-200" {
                    div ."flex flex-col rounded-box gap-4 p-4 bg-base-300" {
                        p ."text-lg" { "Participants" }
                        div #participants ."flex flex-col gap-2 max-w-fit px-4" {
                            @for participant in &self.room.participants {
                                div ."flex items-center rounded-badge bg-base-100"{
                                    div  #(participant.id) .{"avatar placeholder flex " @if is_ready(participant.id) {"online"}} {
                                        div ."bg-neutral text-neutral-content rounded-full w-12" {
                                            span {(participant.name.chars().take(2).collect::<String>())}
                                        }
                                    }
                                    p ."px-4" { (participant.name) }
                                }
                            }
                        }
                    }
                }

                section ."p-8 rounded-box shadow-xl bg-base-200" {
                    @if !is_ready(self.me) {
                        button
                            ."btn btn-success btn-block"
                            hx-put={"/daily/" (self.daily_id) }
                            hx-vals={"{\"ready\": true}"}
                            hx-swap="none"
                        {
                            "I'm so ready"
                        }
                    } @else {
                        button
                            ."btn btn-error btn-block"
                            hx-put={"/daily/" (self.daily_id) }
                            hx-vals={"{\"ready\": false}"}
                            hx-swap="none"
                        {
                            "Wait! I was not ready"
                        }
                    }
                }
            }
        };

        root::layout(body)
    }
}

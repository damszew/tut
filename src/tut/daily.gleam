import gleam/erlang/process.{type Subject}
import gleam/otp/actor
import gleam/list
import gleam/result

pub opaque type Daily {
  Daily(inner: Subject(Message))
}

pub fn new() -> Result(Daily, Nil) {
  actor.start([], handle_message)
  |> result.map(Daily)
  |> result.nil_error()
}

pub fn send(daily: Daily, event: String) -> Nil {
  process.send(daily.inner, Send(event))
}

pub fn join(daily: Daily, participant: Subject(String)) -> Nil {
  process.send(daily.inner, Join(participant))
}

type Participants =
  List(Subject(String))

type Message {
  Join(participant: Subject(String))

  Send(event: String)
}

fn handle_message(
  message: Message,
  participants: Participants,
) -> actor.Next(Message, Participants) {
  case message {
    Send(event) -> {
      participants
      |> list.each(fn(participant) { process.send(participant, event) })

      actor.continue(participants)
    }

    Join(new_participant) -> {
      let new_participants = [new_participant, ..participants]

      actor.continue(new_participants)
    }
  }
}

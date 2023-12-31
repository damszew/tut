import gleam/erlang/process.{type Subject}
import gleam/otp/actor
import gleam/list
import gleam/result

pub type Event {
  NewPersonJoined
  RaisedHand
  PersonLeft
}

pub type Participant =
  Subject(Event)

pub opaque type Daily {
  Daily(inner: Subject(Message))
}

pub fn new() -> Result(Daily, Nil) {
  actor.start([], handle_message)
  |> result.map(Daily)
  |> result.nil_error()
}

pub fn send(daily: Daily, event: Event) -> Nil {
  process.send(daily.inner, Send(event))
}

pub fn join(daily: Daily, participant: Participant) -> Nil {
  process.send(daily.inner, Join(participant))
}

type Participants =
  List(Participant)

type Message {
  Join(participant: Participant)

  Send(event: Event)
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
      participants
      |> list.each(fn(participant) {
        process.send(participant, NewPersonJoined)
      })

      let new_participants = [new_participant, ..participants]

      actor.continue(new_participants)
    }
  }
}

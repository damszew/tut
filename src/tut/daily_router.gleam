import gleam/erlang/process.{type Subject}
import gleam/otp/actor
import gleam/dict
import gleam/result
import tut/daily
import minigen

pub opaque type DailyRouter {
  DailyRouter(inner: Subject(Message))
}

pub fn new() -> Result(DailyRouter, Nil) {
  actor.start(dict.new(), handle_message)
  |> result.map(DailyRouter)
  |> result.nil_error()
}

pub fn create(daily: DailyRouter) -> String {
  let daily_id =
    minigen.string(8)
    |> minigen.run

  // TODO: Actor should create id and return it
  actor.send(daily.inner, Create(daily_id))

  daily_id
}

pub fn join(
  daily: DailyRouter,
  daily_id: String,
  participant: Subject(String),
) -> daily.Daily {
  let assert Ok(r) =
    actor.call(daily.inner, fn(x) { Join(daily_id, participant, x) }, 5000)

  r
}

type Dailies =
  dict.Dict(String, daily.Daily)

type Message {
  Join(
    daily_id: String,
    participant: Subject(String),
    reply_with: Subject(Result(daily.Daily, Nil)),
  )

  Create(daily_id: String)
}

fn handle_message(
  message: Message,
  dailies: Dailies,
) -> actor.Next(Message, Dailies) {
  case message {
    Join(daily_id, new_participant, reply_with) -> {
      let assert Ok(daily) =
        dailies
        |> dict.get(daily_id)

      daily
      |> daily.join(new_participant)

      // TODO: Respond with error if no such daily exists
      process.send(reply_with, Ok(daily))
      actor.continue(dailies)
    }

    Create(daily_id) -> {
      let x =
        dailies
        |> dict.get(daily_id)

      let dailies = case x {
        Ok(_) -> dailies
        Error(_) -> {
          let assert Ok(new_daily) = daily.new()

          let dailies =
            dailies
            |> dict.insert(daily_id, new_daily)

          dailies
        }
      }

      actor.continue(dailies)
    }
  }
}

import wisp
import mist
import gleam/erlang/process
import tut/router
import tut/daily/daily_router

pub fn main() {
  wisp.configure_logger()

  // TODO: Pass key from env
  // Here we generate a secret key, but in a real application you would want to
  // load this from somewhere so that it is not regenerated on every restart.
  let secret_key_base = wisp.random_string(64)

  let assert Ok(daily_router) = daily_router.new()

  let context = router.Context(db: daily_router)
  let handler = router.handle_request(_, context)

  handler
  |> wisp.mist_handler(secret_key_base)
  |> mist.new
  |> mist.port(8000)
  |> mist.start_http

  process.sleep_forever()
}

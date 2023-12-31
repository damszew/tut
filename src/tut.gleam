import wisp
import mist.{type Connection, type ResponseData}
import gleam/http/request.{type Request}
import gleam/http/response.{type Response}
import gleam/erlang/process
import tut/web/router
import tut/web/controllers/daily
import tut/daily_router

pub fn main() {
  wisp.configure_logger()

  // TODO: Pass key from env
  // Here we generate a secret key, but in a real application you would want to
  // load this from somewhere so that it is not regenerated on every restart.
  let secret_key_base = wisp.random_string(64)

  let assert Ok(daily_router) = daily_router.new()

  let context = router.Context(db: daily_router)

  let wisp_handler =
    router.handle_request(_, context)
    |> wisp.mist_handler(secret_key_base)

  // WA: Until https://github.com/gleam-wisp/wisp/issues/10
  fn(req: Request(Connection)) -> Response(ResponseData) {
    case request.path_segments(req) {
      ["daily", daily_id, "ws"] -> {
        daily.daily_websocket(req, daily_id, daily_router)
      }

      _ -> wisp_handler(req)
    }
  }
  |> mist.new
  |> mist.port(8000)
  |> mist.start_http

  process.sleep_forever()
}

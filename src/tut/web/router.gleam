import wisp.{type Request, type Response}
import tut/web/controllers/home
import tut/web/controllers/daily
import tut/daily_router

pub type Context {
  Context(db: daily_router.DailyRouter)
}

pub fn handle_request(req: Request, ctx: Context) -> Response {
  // Apply the middleware stack for this request/response.
  use _req <- middleware(req)

  case wisp.path_segments(req) {
    // This matches `/`.
    [] -> home.page(req)

    ["daily", "new"] -> daily.page_new(req)
    ["daily"] -> daily.create(req, ctx.db)
    ["daily", id] -> daily.join(req, id, ctx.db)

    _ -> wisp.not_found()
  }
}

/// The middleware stack that the request handler uses. The stack is itself a
/// middleware function!
///
/// Middleware wrap each other, so the request travels through the stack from
/// top to bottom until it reaches the request handler, at which point the
/// response travels back up through the stack.
/// 
/// The middleware used here are the ones that are suitable for use in your
/// typical web application.
/// 
pub fn middleware(
  req: wisp.Request,
  handle_request: fn(wisp.Request) -> wisp.Response,
) -> wisp.Response {
  // Permit browsers to simulate methods other than GET and POST using the
  // `_method` query parameter.
  let req = wisp.method_override(req)

  // Log information about the request and response.
  use <- wisp.log_request(req)

  // Return a default 500 response if the request handler crashes.
  use <- wisp.rescue_crashes

  // Rewrite HEAD requests to GET requests and return an empty body.
  use req <- wisp.handle_head(req)

  // Handle the request!
  handle_request(req)
}

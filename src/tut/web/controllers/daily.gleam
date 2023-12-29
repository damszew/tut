import wisp.{type Request, type Response}
import tut/daily_router
import tut/web/views/pages/new_daily
import tut/web/views/pages/join_daily

pub fn page_new(_: Request) -> Response {
  let body = new_daily.page()

  wisp.html_response(body, 200)
}

pub fn create(req: Request, daily_router: daily_router.DailyRouter) -> Response {
  use _formdata <- wisp.require_form(req)

  let token = daily_router.create(daily_router)

  wisp.redirect(to: "daily/" <> token)
}

pub fn join(_: Request, daily_id: String) -> Response {
  let body = join_daily.page(daily_id)
  wisp.html_response(body, 200)
}

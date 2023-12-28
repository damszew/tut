import wisp.{type Request, type Response}
import tut/web/pages/home

pub fn page(_: Request) -> Response {
  let body = home.page()

  wisp.html_response(body, 200)
}

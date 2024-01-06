use crate::web::views::pages::home;

pub async fn page() -> maud::Markup {
    home::page()
}

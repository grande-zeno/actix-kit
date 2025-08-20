pub mod accounts;
pub mod emails;

use tera::Context;
use actix_web::web::Html;
use crate::TERA;

struct GuestPage;

#[actix_web::get("/")]
pub async fn guest_page() -> Html {
    let template = GuestPage::render();

    Html::new(template)
}

impl GuestPage {
    pub fn render() -> String {
        let context = Context::new();

        let tera = TERA.read().unwrap();
        let template = tera.render("views/welcome.html", &context).unwrap();

        template
    }
}

struct Dashboard;

#[actix_web::get("/dashboard")]
pub async fn dashboard() -> Html {
    let template = Dashboard::render();

    Html::new(template)
}

impl Dashboard {
    pub fn render() -> String {
        let context = Context::new();

        let tera = TERA.read().unwrap();
        let template = tera.render("views/dashboard.html", &context).unwrap();

        template
    }
}

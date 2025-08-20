use crate::TERA;

use actix_web::{
    get,
    web::Html,
};

use tera::Context;

struct LoginPage;

impl LoginPage {
    pub fn render() -> String {
        let context = Context::new();
        
        let tera = TERA.read().unwrap();
        let template = tera.render("views/accounts/login.html", &context).unwrap();

        template
    }
}

#[get("/login")]
pub async fn login_page() -> Html {
    let template = LoginPage::render();

    Html::new(template)
}
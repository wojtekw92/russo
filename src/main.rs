#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::outcome::IntoOutcome;
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies};

extern crate rocket_contrib;
extern crate jsonwebtoken as jwt;

use jwt::{encode, decode, Header, Algorithm, Validation};

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};

use serde::{Deserialize, Serialize};

mod password;

// enum FrontendError {
//     WrongPassword,
// }
// impl FrontendError {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             FrontendError::WrongPassword => "Wrong Password",
//         }
//     }
// }




#[derive(FromForm)]
struct Login {
    login: String,
    password: String,
}



#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    login: String,

}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        request.cookies()
            .get("user_id")
            .and_then(|cookie| serde_json::from_str(cookie.value()).ok())
            .or_forward(())
    }
}


#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

#[get("/login")]
fn login_page(flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context = HashMap::<String, String>::new();
    if let Some(ref msg) = flash {
        context.insert("error".to_string(), msg.msg().to_string());
    }
    // context.insert("error".to_string(), flash.map(|msg| format!("{}: {}", msg.name(), msg.msg())).unwrap().to_string());
    Template::render("login", context)
}

#[post("/login", data = "<login>")]
fn do_login(mut cookies: Cookies<'_>, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    let hash_from_db="$2b$12$5Ihqy/23M2qZz2SSzKG18ODILI1F57qU7O7uMlMqFx2.VR4Rn9VQu";// testowe
    let username = "wwozniak92@gmail.com";
    let mut context = HashMap::<String, String>::new();
    context.insert("login".to_string(),login.login.clone());
    context.insert("password".to_string(),login.password.clone());
    if password::check_hash(&login.password, &hash_from_db) && login.login == username {
        let user_data: User = User {
            id: 0,
            login: login.login.clone()
        };
        cookies.add(Cookie::new("user_id", serde_json::to_string(&user_data).unwrap()));
        return Ok(Redirect::to(uri!(status_page)))
    } else {
        return Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username."));
    }
}

#[get("/status")]
fn status_page(user: User) -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert("user_id".to_string(), user.id.to_string());
    context.insert("login".to_string(), user.login);
    Template::render("status", context)
}

#[get("/status", rank = 2)]
fn status_redirect() -> Redirect {
    Redirect::to(uri!(login_page))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![login_page])
        .mount("/", routes![do_login, status_page,status_redirect])
        .attach(Template::fairing()) 
        .launch();
}
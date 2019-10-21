#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::outcome::IntoOutcome;
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies};

extern crate rocket_contrib;


use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};

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


#[derive(Debug)]
struct User(usize);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
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
         cookies.add_private(Cookie::new("user_id", 1.to_string()));
         Ok(Redirect::to(uri!(status_page)))

    } else {
        return Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username."));
    }

    //  Ok(Template::render("status", context))
}

#[get("/status")]
fn status_page(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("user_id", user.0);
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
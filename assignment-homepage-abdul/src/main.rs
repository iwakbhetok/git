use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use serde::Deserialize;
use actix_web::http::header::ContentType;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

// Hardcoded credentials
const VALID_USERNAME: &str = "admin";
const VALID_PASSWORD: &str = "secret";

#[get("/")]
async fn home(session: Session) -> impl Responder {
    if let Some(username) = session.get::<String>("username").unwrap() {
        HttpResponse::Ok().content_type(ContentType::html()).body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Home</title>
                <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet">
            </head>
            <body>
                <div class="container mt-5">
                    <h1 class="mb-4">Welcome to the home page, <b>{username}</b>!</h1>
                    <a href="/profile" class="btn btn-primary me-2">View Profile</a>
                    <form action="/logout" method="post" class="d-inline">
                        <button type="submit" class="btn btn-danger">Logout</button>
                    </form>
                </div>
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/js/bootstrap.bundle.min.js"></script>
            </body>
            </html>
            "#
        ))
    } else {
        HttpResponse::Ok().content_type(ContentType::html()).body(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Home</title>
                <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet">
            </head>
            <body>
                <div class="container mt-5">
                    <h1 class="mb-4">Welcome to the home page!</h1>
                    <a href="/login" class="btn btn-primary">Login</a>
                </div>
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/js/bootstrap.bundle.min.js"></script>
            </body>
            </html>
            "#
        )
    }
}

#[get("/profile")]
async fn profile(session: Session) -> impl Responder {
    if let Some(username) = session.get::<String>("username").unwrap() {
        HttpResponse::Ok().content_type(ContentType::html()).body(format!(
            r#"
            <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Profile</title>
                <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet">
            </head>
                <body>
                    <div class="container mt-5">
                    <h1>Profile</h1>
                    <p>This is the profile page for {}.</p>
                    <form action="/logout" method="post">
                        <input type="submit" value="Logout" class="btn btn-primary">
                    </form>
                    </div>
                </body>
            </html>
            "#,
            username
        ))
    } else {
        HttpResponse::Unauthorized().content_type(ContentType::html()).body(
            r#"
            <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Unauthorized</title>
                <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet">
            </head>
                <body>
                    <div class="container mt-5">
                        <h1>Unauthorized</h1>
                        <p>Please <a href="/login" class="btn btn-primary">login</a> to view your profile.</p>
                    </div>
                </body>
            </html>
            "#
        )
    }
}

#[get("/login")]
async fn login_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Login</title>
                <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet">
            </head>
            <body>
                <div class="container mt-5">
                    <h1 class="mb-4">Login</h1>
                    <form action="/login" method="post">
                        <div class="mb-3">
                            <label for="username" class="form-label">Username:</label>
                            <input type="text" class="form-control" id="username" name="username" required>
                        </div>
                        <div class="mb-3">
                            <label for="password" class="form-label">Password:</label>
                            <input type="password" class="form-control" id="password" name="password" required>
                        </div>
                        <button type="submit" class="btn btn-primary">Login</button>
                    </form>
                </div>
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/js/bootstrap.bundle.min.js"></script>
            </body>
            </html>
            "#
        )
}

#[post("/login")]
async fn login(form: web::Form<LoginForm>, session: Session) -> impl Responder {
    if form.username == VALID_USERNAME && form.password == VALID_PASSWORD {
        session.insert("username", &form.username).unwrap();
        HttpResponse::SeeOther().append_header(("Location", "/"))
        .finish()
    } else {
        HttpResponse::Unauthorized().body(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Login</title>
                <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet">
            </head>
            <body>
                <div class="container mt-5">
                    <div class="alert alert-danger" role="alert">
                    Invalid Credentials!
                    </div>
                    <a href="/login" class="btn btn-primary">Try Again</a>
                </div>
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/js/bootstrap.bundle.min.js"></script>
            </body>
            </html>
            "#
        )
    }
}

#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::SeeOther().insert_header(("Location", "/")).finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::from(&[0; 64]), // In production, use a proper secret key
            ))
            .service(home)
            .service(profile)
            .service(login_page)
            .service(login)
            .service(logout)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
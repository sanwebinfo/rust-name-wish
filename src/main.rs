use actix_web::{web, App, HttpServer, HttpResponse, Error};
use actix_files::Files;
use tera::{Tera, Context};
use serde::Deserialize;
use slug::slugify;

#[derive(Deserialize)]
struct QueryParams {
    name: String,
}

async fn greet(query: web::Query<QueryParams>, tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let name = query.name.trim().is_empty().then(|| "Your Name").unwrap_or(query.name.trim());

    if name.is_empty() {
        return missing_name_error(tmpl).await;
    }

    // Data validation
    if name.is_empty() || name.len() > 50 {
        let mut ctx = Context::new();
        ctx.insert("error", "Invalid name provided.");
        
        // Render the error page
        let rendered = tmpl.render("error.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error rendering error page"))?;

        return Ok(HttpResponse::BadRequest()
            .content_type("text/html")
            .insert_header(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
            .insert_header(("X-Frame-Options", "DENY"))
            .insert_header(("X-Content-Type-Options", "nosniff"))
            .insert_header(("X-XSS-Protection", "1; mode=block"))
            .insert_header(("X-Robots-Tag", "noindex, nofollow"))
            .body(rendered));
    }

    let slugified_name = slugify(name);

    if name != slugified_name {
        return Ok(HttpResponse::Found()
            .append_header(("Location", format!("/wish?name={}", slugified_name)))
            .insert_header(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
            .insert_header(("X-Frame-Options", "DENY"))
            .insert_header(("X-Content-Type-Options", "nosniff"))
            .insert_header(("X-XSS-Protection", "1; mode=block"))
            .insert_header(("X-Robots-Tag", "noindex, nofollow"))
            .finish());
    }

    let display_name = name.replace("-", " ");

    // Prepare context for the template
    let mut ctx = Context::new();
    ctx.insert("name", &display_name);
    ctx.insert("name_slug", &slugified_name);
    let canonical_url = format!("/wish?name={}", slugified_name);
    ctx.insert("canonical_url", &canonical_url);

    // Render the template and handle errors
    let rendered = tmpl.render("index.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template rendering failed"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .insert_header(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
        .insert_header(("X-Frame-Options", "DENY"))
        .insert_header(("X-Content-Type-Options", "nosniff"))
        .insert_header(("X-XSS-Protection", "1; mode=block"))
        .insert_header(("X-Robots-Tag", "noindex, nofollow"))
        .body(rendered))
}

async fn home(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("message", "Greeting Maker 🎉");

    // Set the canonical URL for the home page
    let canonical_url = format!("/");
    ctx.insert("canonical_url", &canonical_url);

    // Render the home page
    let rendered = tmpl.render("home.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template rendering failed"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .insert_header(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
        .insert_header(("X-Frame-Options", "DENY"))
        .insert_header(("X-Content-Type-Options", "nosniff"))
        .insert_header(("X-XSS-Protection", "1; mode=block"))
        .insert_header(("X-Robots-Tag", "noindex, nofollow"))
        .body(rendered))
}

async fn not_found(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("error", "Page not found.");

    let rendered = tmpl.render("error.html", &ctx)
       .map_err(|_| actix_web::error::ErrorInternalServerError("Template rendering failed"))?;

    Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .insert_header(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
        .insert_header(("X-Frame-Options", "DENY"))
        .insert_header(("X-Content-Type-Options", "nosniff"))
        .insert_header(("X-XSS-Protection", "1; mode=block"))
        .insert_header(("X-Robots-Tag", "noindex, nofollow"))
        .body(rendered))
}

async fn missing_name_error(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("error", "Missing required field name - Please provide a valid name.");

    let rendered = tmpl.render("error.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template rendering failed"))?;

    Ok(HttpResponse::BadRequest()
       .content_type("text/html")
       .insert_header(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
       .insert_header(("X-Frame-Options", "DENY"))
       .insert_header(("X-Content-Type-Options", "nosniff"))
       .insert_header(("X-XSS-Protection", "1; mode=block"))
       .insert_header(("X-Robots-Tag", "noindex, nofollow"))
       .body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").expect("Error initializing Tera templates");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            // Serve static files (images, CSS, JS, etc.)
            .service(web::scope("/static").service(Files::new("/", "./static")))
            .route("/", web::get().to(home))
            .route("/wish", web::get().to(greet))
            .route("/404", web::get().to(not_found))
            .route("/error", web::get().to(missing_name_error))
            .default_service(web::route().to(not_found))
    })
    .bind("127.0.0.1:6022")?
    .run()
    .await
}

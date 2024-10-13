use actix_web::http;
use actix_web::{web,HttpResponse,Error,Result,error};
use tera::{Tera, Context};

fn redirect(location : &str) -> HttpResponse{
    HttpResponse::Found()
        .append_header((http::header::LOCATION,location))
        .finish()
    
}

pub async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse,Error>{
    let mut ctx = Context::new();
    let a = tmpl
        .render("index.html",&ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}


pub async fn login(tmpl: web::Data<Tera>) -> Result<HttpResponse,Error>{
    let mut ctx = Context::new();
    let a = tmpl
        .render("login.html",&ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

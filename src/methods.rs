use std::net::Ipv4Addr;
use std::str::FromStr;

use actix_web::{web, get, HttpResponse, Responder};

use crate::app_state::AppState;

#[get("/{ip}")]
pub async fn get(path: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let ip = path.into_inner();
    
    match Ipv4Addr::from_str(&ip) {
        Ok(ipv4) => ipv4,
        Err(_error) => return HttpResponse::BadRequest().body("Not a IPv4"),
    };

    let ips = &app_state.ips;
    HttpResponse::Ok().body(ips.contains(&ip).to_string())
}
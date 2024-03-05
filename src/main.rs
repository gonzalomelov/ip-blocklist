use std::env;
use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::str::FromStr;

use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};

use ip_blocklist::{read_from_file_to_hash_set, app_state::AppState};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut filename = "ips.csv";
    if args.len() > 1 {
        filename = &args[1];
    }
    
    let ips: HashSet<String> = match read_from_file_to_hash_set(&filename) {
        Ok(hs) => hs,
        Err(error) => panic!("Problem reading the file: {:?}", error),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { ips: ips.clone() }))    
            .service(web::scope("/ips").service(get))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
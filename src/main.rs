use actix_web::{web, App, HttpServer};

mod controllers;
mod models;

fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        controllers::health::health
    );
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    HttpServer::new(|| { 
        App::new().service(web::scope("/njord-api").configure(config_routes))
    
    }).bind(("0.0.0.0", 8080))?.run().await

}

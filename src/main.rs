use actix_web::{App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use solo_project::infra;
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let ip = env::var("IP").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let dir_static = env::var("DIR_STATIC").unwrap();
    let path_static = env::var("PATH_STATIC").unwrap();
    let cors = env::var("CORS_REACT").unwrap();
    // println!("{}", cors);
    HttpServer::new(move || {
        App::new().wrap(
            Cors::default()
                .allowed_origin(&cors) 
                .allow_any_method() 
                .allow_any_header() 
                .supports_credentials()
                .max_age(3600)
        )
            .configure(infra::askan::ascan::conf_ascan)
            .service(fs::Files::new(&path_static, &dir_static).show_files_listing())
    })
    .workers(10)
    .bind((ip, port))?
    .run()
    .await
    
}
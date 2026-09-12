use actix_files::Files;
use actix_web::{App, HttpServer, web};

use crate::{app::KM, routes::main_routes::{index, login, register, register_post}};

pub async fn server(km: KM) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(km.clone()))
            .service(index)
            .service(register)
            .service(register_post)
            .service(login)
            .service(Files::new("/css", "./public/css"))
            .service(Files::new("/js", "./public/js"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

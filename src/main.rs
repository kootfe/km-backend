mod app;
mod db;
mod routes;
mod web_pages;
mod data;
mod http;
mod services;

use crate::{
    app::{KM, server::server, trace::set_trace},
};

use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let args: Vec<String> = std::env::args().collect();
    set_trace(&args);
    let km = KM::build_auto().await?;
    server(km).await?;
    Ok(())
}

use actix_web::{get, web, App, HttpServer, HttpResponse};
use deadpool_redis::{cmd, Pool};
use dotenv::dotenv;

mod config;
mod models;

#[get("/")]
async fn index(pool: web::Data<Pool>) -> Result<HttpResponse, models::Error> {
    let mut conn = pool.get().await?;


    let retrieved_value: Vec<String> = cmd("KEYS").arg("*").query_async(&mut conn).await.unwrap();
    Ok(HttpResponse::Ok().json(retrieved_value))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let cfg = config::Config::from_env().unwrap();
    let pool = cfg.redis.create_pool().unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(index)
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}

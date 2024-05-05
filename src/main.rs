use std::sync::Mutex;
use std::time::Duration;
use actix_web::{App, get, HttpResponse, HttpServer, web};
use actix_web::http::header::ContentType;
use clap::Parser;
use serde_derive::Deserialize;
use crate::structs::Errors;

mod cache;
mod structs;
mod rss;
mod config;
mod fetch;

struct Storage {
    cache: Mutex<cache::Cache>,
    config: config::Config,
}

#[derive(Deserialize)]
struct QueryParams {
    location: Option<u32>,
    category: Option<u32>,
}

#[get("/genre/{genre}")]
async fn genre_handler(genre: web::Path<String>, params: web::Query<QueryParams>, data: web::Data<Storage>) -> HttpResponse {
    let record = structs::GenreRecord {
        genre: genre.split('+').map(|v| v.to_string()).collect(),
        location: params.location,
        category: params.category,
    };
    let record_type = structs::RecordType::GenreRecord(record.clone());
    let cache = data.cache.lock().unwrap();
    let result = if let Some(result) = cache.get(&record_type) {
        result
    } else {
        drop(cache);
        let result = fetch::get_discover_response(record.clone()).await;
        {
            let mut cache = data.cache.lock().unwrap();
            cache.insert(record_type, result.clone());
        }
        result
    };

    match result {
        Ok(result) => {
            let feed = rss::channel_from_discover_response(record, result, &data.config);
            HttpResponse::Ok().body(feed.to_string())
        }
        Err(Errors::InternalError) => {
            HttpResponse::InternalServerError().body("Internal server error!")
        }
        Err(Errors::NotFound) => {
            HttpResponse::NotFound().body("Not found!")
        }
    }
}

#[get("/")]
async fn slash() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType(mime::TEXT_HTML))
        .body(include_str!("../resources/index.html"))
}

#[get("/index.html")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType(mime::TEXT_HTML))
        .body(include_str!("../resources/index.html"))
}

#[get("/script.js")]
async fn script() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType(mime::TEXT_JAVASCRIPT))
        .body(include_str!("../resources/script.js"))
}

fn cleanup(cache: &Mutex<cache::Cache>) {
    if let Ok(mut cache) = cache.lock() {
        println!("Cleaning up stale data");
        cache.cleanup()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::parse();
    let data = web::Data::new(Storage {
        cache: Mutex::new(cache::Cache::new(config.cache_age)),
        config,
    });
    let data_clone = data.clone();

    actix_rt::spawn(async move {
        let mut interval = actix_rt::time::interval(Duration::from_secs(60 * 60));
        loop {
            interval.tick().await;
            cleanup(&data_clone.cache)
        }
    });

    HttpServer::new(move || {
        App::new()
            .service(genre_handler)
            .service(slash)
            .service(index)
            .service(script)
            .app_data(data.clone())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

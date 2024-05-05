use reqwest::{Client, StatusCode};
use super::structs::{Errors, GenreRecord, QueryResult};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize)]
struct DiscoverRequest {
    category_id: u32,
    tag_norm_names: Vec<String>,
    geoname_id: u32,
    slice: &'static str,
    cursor: &'static str,
    size: u32,
    include_result_types: [&'static str; 2]
}

#[derive(Deserialize)]
struct Response {
    results: Vec<QueryResult>,
}

pub async fn get_discover_response(record: GenreRecord) -> Result<Vec<QueryResult>, Errors> {
    let client = Client::new();
    let request = DiscoverRequest {
        category_id: record.category.unwrap_or(0),
        tag_norm_names: record.genre.clone(),
        geoname_id: record.location.unwrap_or(0),
        slice: "new",
        cursor: "*",
        size: 60,
        include_result_types: ["a", "s"]
    };
    let res = client.post("https://bandcamp.com/api/discover/1/discover_web")
        .body(serde_json::to_string(&request).unwrap())
        .header("Content-Type", "application/json; charset=UTF-8")
        .send()
        .await
        .map_err(|err| {
            println!("inernal error: {err}");
            Errors::InternalError
        })?;
    if res.status() == StatusCode::NOT_FOUND {
        return Err(Errors::NotFound)
    }
    if res.status() != StatusCode::OK {
        println!("internal error: {res:?}");
        return Err(Errors::InternalError)
    }
    let text = res.text().await.map_err(|err| {
        println!("inernal error: {err}");
        Errors::InternalError
    })?;
    let response: Response = serde_json::from_str(&text).map_err(|err| {
        println!("Internal error: {err}");
        Errors::InternalError
    })?;
    Ok(response.results)
}


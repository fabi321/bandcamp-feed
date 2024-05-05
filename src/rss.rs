use super::structs::{QueryResult, GenreRecord};
use super::config::Config;
use rss::{Item, ItemBuilder, Channel, ChannelBuilder, Enclosure};

pub fn item_from_discover_response(result: QueryResult) -> Item {
    let image_url = format!("https://f4.bcbits.com/img/{}{}_2.jpg", result.result_type, result.item_image_id);
    let runtime = match result.item_duration {
        None => "no".to_string(),
        Some(i) if i < 100.0  => {
            format!("{i:0.0} seconds")
        }
        Some(i) if i < 3570.0 => {
            format!("{:0.0} minutes", i / 60.0)
        }
        Some(i) => {
            format!("{:0.0} hours", i / 3600.0)
        }
    };
    let location = result.band_location.unwrap_or_else(|| "Unknown".to_string());
    let content = format!(r#"<p>{title}<br>from {artist}<br>from {location}<br>{tracks} tracks with {runtime} runtime<br>for {price:0.2} {currency}</p><img src="{image_url}"/>"#, title=result.title, artist=result.band_name, tracks=result.track_count.unwrap_or(0), price=result.item_price, currency=result.item_currency);
    ItemBuilder::default()
        .title(format!("{} by {}", result.title, result.band_name))
        .link(result.item_url)
        .author(result.band_name)
        .pub_date(result.release_date.format("%a, %d %b %Y %H:%M:%S %Z").to_string())
        .enclosure(Enclosure {
            url: image_url,
            length: "1000".to_string(),
            mime_type: "image/jpeg".to_string(),
        })
        .content(content)
        .build()
}

pub fn channel_from_discover_response(record_type: GenreRecord, result: Vec<QueryResult>, config: &Config) -> Channel {
    let items: Vec<_> = result.into_iter().map(item_from_discover_response).collect();
    ChannelBuilder::default()
        .title(format!("Newest albums for genre {}", record_type.genre.join(", ")))
        .link(format!("{}/genre/{}?location={}&category={}", config.own_url, record_type.genre.join("+"), record_type.location.unwrap_or(0), record_type.category.unwrap_or(0)))
        .items(items)
        .build()
}


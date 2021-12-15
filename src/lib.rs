extern crate rss;
extern crate serde_json;

use serde_json::Value;
use rss::{ChannelBuilder, ItemBuilder, Item, EnclosureBuilder, GuidBuilder};

pub fn generate_channel<R: std::io::Read>(reader: R, item_key: &str, id_key: &str, title_key: &str, image_key: &Option<String>) -> Result<String, ParseError> {
    let parsed: Value = serde_json::from_reader(reader)?;
    let items = &parsed[item_key].as_array().ok_or(ParseError::ItemArrayIncorrect)?;
    let rss_items: Result<Vec<_>, _> = items.into_iter().map(|item| create_item(item, id_key, title_key, image_key)).collect();
    let valid_items: Vec<_> = rss_items?;
    let channel = ChannelBuilder::default()
        .title("Test channel")
        .items(valid_items)
        .build();
    return Ok(channel.to_string());
}

pub fn create_item(item: &Value, id_key: &str, title_key: &str, image_key: &Option<String>) -> Result<Item, ParseError> {
    let guid = item[id_key].as_str().ok_or(ParseError::CouldNotParseId)?;
    let title = item[title_key].as_str().ok_or(ParseError::CouldNotParseTitle)?;
    let mut rss_item = ItemBuilder::default()
        .guid(GuidBuilder::default().value(guid).build())
        .title(title.to_string())
        .build();
    if let Some(image_key) = image_key {
        let image_url = item[image_key].as_str().ok_or(ParseError::CouldNotParseImage)?;
        let image = EnclosureBuilder::default()
            .url(image_url)
            .mime_type("image/jpeg")
            .build();
        rss_item.set_enclosure(image);
    }
    return Ok(rss_item);
}

#[derive(Debug)]
pub enum ParseError {
    ItemArrayIncorrect,
    CouldNotParseId,
    CouldNotParseTitle,
    CouldNotParseImage,
    JSONError(serde_json::Error),
}

impl From<serde_json::Error> for ParseError {
    fn from(serde_err: serde_json::Error) -> Self {
        return Self::JSONError(serde_err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn care_by_volvo() {
        let file = std::fs::File::open("testdata/CbV.json").unwrap();
        let res = generate_channel(&file, "data", "title", "image");
        assert!(res.is_ok());
    }
}

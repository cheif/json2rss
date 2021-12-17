extern crate rss;
extern crate serde;
extern crate serde_json;
extern crate handlebars;
extern crate reqwest;

use serde::Deserialize;
use serde_json::Value;
use rss::{ChannelBuilder, ItemBuilder, Item, EnclosureBuilder, GuidBuilder};
use handlebars::Handlebars;

#[derive(Deserialize)]
pub struct Config {
    source: String,
    link: String,
    title: String,
    item_key: String,
    url_template: String,
    title_template: String,
    description_template: String,
    image_key: Option<String>
}

pub fn generate_channel(config: &Config) -> Result<String, Error> {
    let res = reqwest::blocking::get(&config.source)?;
    let parsed: Value = serde_json::from_reader(res)?;
    let items = &parsed[&config.item_key].as_array().ok_or(Error::ItemArrayIncorrect)?;
    let rss_items: Result<Vec<_>, _> = items.into_iter().map(|item| create_item(item, config)).collect();
    let valid_items: Vec<_> = rss_items?;
    let channel = ChannelBuilder::default()
        .title(&config.title)
        .link(&config.link)
        .items(valid_items)
        .build();
    return Ok(channel.to_string());
}

pub fn create_item(item: &Value, config: &Config) -> Result<Item, Error> {
    let url = render(&config.url_template, item)?;
    let title = render(&config.title_template, item)?;
    let description = render(&config.description_template, item)?;
    let mut rss_item = ItemBuilder::default()
        .guid(GuidBuilder::default().value(url).build())
        .title(title)
        .description(description)
        .build();
    if let Some(image_key) = &config.image_key {
        let image_url = item[image_key].as_str().ok_or(Error::CouldNotParseImage)?;
        let mut image = EnclosureBuilder::default()
            .url(image_url)
            .mime_type("image/jpeg")
            .build();
        if let Some(length) = reqwest::blocking::get(image_url).ok().and_then(|c| c.content_length()) {
            image.set_length(format!("{}", length));
        }
        rss_item.set_enclosure(image);
    }
    return Ok(rss_item);
}

fn render(template_str: &str, item: &Value) -> Result<String, Error> {
    let handlebars = Handlebars::new();
    let res = handlebars.render_template(template_str, item);
    return Ok(res?);
}

#[derive(Debug)]
pub enum Error {
    ItemArrayIncorrect,
    TemplateError(handlebars::TemplateError),
    RenderError(handlebars::RenderError),
    CouldNotParseId,
    CouldNotParseImage,
    JSONError(serde_json::Error),
    FetchError(reqwest::Error)
}

impl From<serde_json::Error> for Error {
    fn from(serde_err: serde_json::Error) -> Self {
        return Self::JSONError(serde_err);
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Self {
        return Self::TemplateError(err);
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Self {
        return Self::RenderError(err);
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        return Self::FetchError(err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn care_by_volvo() {
        let file = std::fs::File::open("testdata/CbV.json").unwrap();
        let config = Config {
            source: "https://www.volvocars.com/api/care-by-volvo/cars/cars/?customerType=b2c&filters.delivery=stock&itemsPerPage=18&market=se&page=1".to_string(),
            link: "https://www.volvocars.com/se/care-by-volvo/cars/".to_string(),
            title: "Care by Volvo".to_string(),
            item_key: "data".to_string(),
            url_template: "https://www.volvocars.com/se/care-by-volvo/cars/{{vehicleId}}/".to_string(),
            title_template: "{{title}} ({{engineType}})".to_string(),
            description_template: "{{basePrice}}:-/Mån\n{{engineDescription}}\n{{#each environmentalDataDetails.wltp}}{{this.label}}: {{this.value}}\n{{/each}}\n\nUppskattad leverans: {{estimateDeliveryDate}}".to_string(),
            image_key: Some("image".to_string())
        };
        let res = generate_channel(&config);
        println!("{:?}", res);
        assert!(res.is_ok());
    }
}

extern crate handlebars;
extern crate reqwest;
extern crate rss;
extern crate serde;
extern crate serde_json;

use handlebars::Handlebars;
use rss::{ChannelBuilder, GuidBuilder, Item, ItemBuilder};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Config {
    source: String,
    link: String,
    title: String,
    item_key: String,
    url_template: String,
    title_template: String,
    description_template: String,
}

pub fn generate_channel(config: &Config) -> Result<String, Error> {
    return _generate_channel(config, reqwest::blocking::Client::new());
}

trait Fetcher {
    fn get(&self, source: &str) -> Result<Box<dyn std::io::Read>, reqwest::Error>;
}
impl Fetcher for reqwest::blocking::Client {
    fn get(&self, source: &str) -> Result<Box<dyn std::io::Read>, reqwest::Error> {
        return Ok(Box::new(self.get(source).send()?));
    }
}


fn _generate_channel(config: &Config, fetcher: impl Fetcher) -> Result<String, Error> {
    let res = fetcher.get(&config.source)?;
    let parsed: Value = serde_json::from_reader(res)?;
    let items = &parsed[&config.item_key]
        .as_array()
        .ok_or(Error::ItemArrayIncorrect)?;
    let rss_items: Result<Vec<_>, _> = items
        .into_iter()
        .map(|item| create_item(item, config))
        .collect();
    let valid_items: Vec<_> = rss_items?;
    let channel = ChannelBuilder::default()
        .title(&config.title)
        .link(&config.link)
        .items(valid_items)
        .build();
    return Ok(channel.to_string());
}

fn create_item(item: &Value, config: &Config) -> Result<Item, Error> {
    let url = render(&config.url_template, item)?;
    let title = render(&config.title_template, item)?;
    let description = render(&config.description_template, item)?;
    return Ok(ItemBuilder::default()
        .guid(GuidBuilder::default().value(url).build())
        .title(title)
        .description(description)
        .build());
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
    JSONError(serde_json::Error),
    FetchError(reqwest::Error),
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
        let config = Config {
            source: "https://www.volvocars.com/api/care-by-volvo/cars/cars/?customerType=b2c&filters.delivery=stock&itemsPerPage=18&market=se&page=1".to_string(),
            link: "https://www.volvocars.com/se/care-by-volvo/cars/".to_string(),
            title: "Care by Volvo".to_string(),
            item_key: "data".to_string(),
            url_template: "https://www.volvocars.com/se/care-by-volvo/cars/{{vehicleId}}/".to_string(),
            title_template: "{{title}} ({{engineType}})".to_string(),
            description_template: "{{basePrice}}:-/Mån\n{{engineDescription}}\n{{#each environmentalDataDetails.wltp}}{{this.label}}: {{this.value}}\n{{/each}}\n\nUppskattad leverans: {{estimateDeliveryDate}}".to_string()
        };
        let res = _generate_channel(&config, TestFetcher{});
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    struct TestFetcher {}
    impl Fetcher for TestFetcher {
        fn get(&self, _source: &str) -> Result<Box<dyn std::io::Read>, reqwest::Error> {
            let file = std::fs::File::open("testdata/CbV.json").unwrap();
            return Ok(Box::new(file));
        }
    }
}

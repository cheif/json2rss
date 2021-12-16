extern crate rss;
extern crate serde_json;
extern crate handlebars;
extern crate reqwest;

use serde_json::Value;
use rss::{ChannelBuilder, ItemBuilder, Item, EnclosureBuilder, GuidBuilder};
use handlebars::Handlebars;

pub fn generate_channel<R: std::io::Read>(reader: R, link: &str, title: &str, item_key: &str, url_template: &str, title_template: &str, description_template: &str, image_key: &Option<String>) -> Result<String, ParseError> {
    let parsed: Value = serde_json::from_reader(reader)?;
    let items = &parsed[item_key].as_array().ok_or(ParseError::ItemArrayIncorrect)?;
    let rss_items: Result<Vec<_>, _> = items.into_iter().map(|item| create_item(item, url_template, title_template, description_template, image_key)).collect();
    let valid_items: Vec<_> = rss_items?;
    let channel = ChannelBuilder::default()
        .title(title.to_string())
        .link(link.to_string())
        .items(valid_items)
        .build();
    return Ok(channel.to_string());
}

pub fn create_item(item: &Value, url_template: &str, title_template: &str, description_template: &str, image_key: &Option<String>) -> Result<Item, ParseError> {
    let url = render(url_template, item)?;
    let title = render(title_template, item)?;
    let description = render(description_template, item)?;
    let mut rss_item = ItemBuilder::default()
        .guid(GuidBuilder::default().value(url).build())
        .title(title)
        .description(description)
        .build();
    if let Some(image_key) = image_key {
        let image_url = item[image_key].as_str().ok_or(ParseError::CouldNotParseImage)?;
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

fn render(template_str: &str, item: &Value) -> Result<String, ParseError> {
    let handlebars = Handlebars::new();
    let res = handlebars.render_template(template_str, item);
    return Ok(res?);
}

#[derive(Debug)]
pub enum ParseError {
    ItemArrayIncorrect,
    TemplateError(handlebars::TemplateError),
    RenderError(handlebars::RenderError),
    CouldNotParseId,
    CouldNotParseImage,
    JSONError(serde_json::Error)
}

impl From<serde_json::Error> for ParseError {
    fn from(serde_err: serde_json::Error) -> Self {
        return Self::JSONError(serde_err);
    }
}

impl From<handlebars::TemplateError> for ParseError {
    fn from(err: handlebars::TemplateError) -> Self {
        return Self::TemplateError(err);
    }
}

impl From<handlebars::RenderError> for ParseError {
    fn from(err: handlebars::RenderError) -> Self {
        return Self::RenderError(err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn care_by_volvo() {
        let file = std::fs::File::open("testdata/CbV.json").unwrap();
        let res = generate_channel(&file, "https://www.volvocars.com/se/care-by-volvo/cars/", "Care by Volvo", "data", "https://www.volvocars.com/se/care-by-volvo/cars/{{vehicleId}}/", "{{title}} ({{engineType}})", "{{basePrice}}:-/Mån\n{{engineDescription}}\n{{#each environmentalDataDetails.wltp}}{{this.label}}: {{this.value}}\n{{/each}}\n\nUppskattad leverans: {{estimateDeliveryDate}}", &Some("image".to_string()));
        println!("{:?}", res);
        assert!(res.is_ok());
    }
}

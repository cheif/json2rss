#[macro_use]
extern crate rouille;
extern crate rss;
extern crate serde_json;
extern crate reqwest;
extern crate handlebars;

use rouille::Request;
use rouille::Response;

mod lib;

fn main() {
    println!("Starting server");
    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request,
                (GET) (/) => {
                    match process_request(request) {
                        Ok(rss) => Response::from_data("application/rss+xml", rss),
                        Err(error) => Response::text(format!("Error: {:?}", error)).with_status_code(400)
                    }
                },
                _ => Response::empty_404()
                )
    });
}

fn process_request(request: &Request) -> Result<String, Error> {
    let source_url = request.get_param("source").ok_or(Error::SourceURLMissing)?;
    let link = request.get_param("link").ok_or(Error::LinkMissing)?;
    let title = request.get_param("title").ok_or(Error::TitleMissing)?;
    let items_key = request.get_param("items_key").ok_or(Error::ItemsKeyMissing)?;
    let url_template = request.get_param("url_template").ok_or(Error::UrlTemplateMissing)?;
    let title_template = request.get_param("title_template").ok_or(Error::TitleTemplateMissing)?;
    let description_template = request.get_param("description_template").ok_or(Error::DescriptionTemplateMissing)?;
    let image_key = request.get_param("image_key");
    let res = reqwest::blocking::get(source_url)?;
    return lib::generate_channel(res, &link, &title, &items_key, &url_template, &title_template, &description_template, &image_key).map_err(|err| Error::ParseError(err));
}

#[derive(Debug)]
pub enum Error {
    SourceURLMissing,
    LinkMissing,
    TitleMissing,
    ItemsKeyMissing,
    UrlTemplateMissing,
    TitleTemplateMissing,
    DescriptionTemplateMissing,
    FetchError(reqwest::Error),
    ParseError(lib::ParseError),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        return Self::FetchError(err);
    }
}

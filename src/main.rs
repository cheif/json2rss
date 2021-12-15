#[macro_use]
extern crate rouille;
extern crate rss;
extern crate serde_json;
extern crate reqwest;

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
    let items_key = request.get_param("items_key").ok_or(Error::ItemsKeyMissing)?;
    let id_key = request.get_param("id_key").ok_or(Error::IdKeyMissing)?;
    let title_key = request.get_param("title_key").ok_or(Error::TitleKeyMissing)?;
    let image_key = request.get_param("image_key");
    let res = reqwest::blocking::get(source_url)?;
    return lib::generate_channel(res, &items_key, &id_key, &title_key, &image_key).map_err(|err| Error::ParseError(err));
}


#[derive(Debug)]
pub enum Error {
    SourceURLMissing,
    ItemsKeyMissing,
    IdKeyMissing,
    TitleKeyMissing,
    FetchError(reqwest::Error),
    ParseError(lib::ParseError),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        return Self::FetchError(err);
    }
}

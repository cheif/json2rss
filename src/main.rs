use rouille::{router, Request, Response};

mod lib;

fn main() {
    println!("Starting server");
    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request,
        (GET) (/feed) => {
            match generate_feed(request) {
                Ok(rss) => Response::from_data("application/rss+xml", rss),
                Err(error) => Response::text(format!("Error: {:?}", error)).with_status_code(400)
            }
        },
        (GET) (/) => {
            Response::from_file("text/html", std::fs::File::open("public/index.html").unwrap())
        },
        _ => {
            let response = rouille::match_assets(&request, "public");
            return if response.is_success() { response } else { Response::empty_404() }
        }
        )
    });
}

fn generate_feed(request: &Request) -> Result<String, Error> {
    let config_str = request.get_param("config").ok_or(Error::ConfigMissing)?;
    let config = serde_json::from_str(&config_str)?;
    return Ok(lib::generate_channel(&config)?);
}

#[derive(Debug)]
pub enum Error {
    ConfigMissing,
    ConfigParseError(serde_json::Error),
    GeneratorError(lib::Error),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::ConfigParseError(err)
    }
}

impl From<lib::Error> for Error {
    fn from(err: lib::Error) -> Self {
        Self::GeneratorError(err)
    }
}

# Convert any JSON API into a RSS stream

This is a small tool that allows you to transforma JSON API into a RSS stream written in rust.


## Running
The easiest way to get started is to just build the provided dockerfile, and then run it exposing port 8080, eg:

```shell
docker build -t json2rss .
docker run -p 8080:8080 json2rss
```

This will then start the service locally, if you've got rust installed you can run natively as well (used for development)

```
cargo run
```

## Using

When the server is up and running you just need to call it with a config-object as a query parameter to get the RSS feed. The format of the config object is defined in `src/lib.rs`, and should be pretty self-explanatory. The config is the passed as a urlencoded json-string to the API.

## TODO
- [ ] Build a simple web-UI for generating the final URL.

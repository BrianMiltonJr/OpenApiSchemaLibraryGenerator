use serde_json;
use openapiv3::OpenAPI;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    file: String,
}
fn main() {
    let args = Args::parse();

    let openapi = parse_open_api(args.file);
    for ele in openapi.paths {
        let route = ele.0;
        let path_item = ele.1.into_item()
            .expect("Unable to gather Path Item");

        println!("\r\n{:?}", route);
        println!("{:?}", path_item.parameters);
    }
}

fn parse_open_api(path: String) -> OpenAPI {
    let data = fs::read_to_string(path)
        .expect("Unable to read file");
    
    let openapi: OpenAPI = serde_json::from_str(&data)
        .expect("Could not deserialize input");

    return openapi;
}
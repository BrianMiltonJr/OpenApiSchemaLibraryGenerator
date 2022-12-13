mod php_gen;
use php_gen::{PHPClass, PHPClassProperty, PHPFunction, PHPAccessModifier};
use std::fs;
use serde_json;
use openapiv3::{OpenAPI, PathItem};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    file: String,
}
fn main() {
    let args = Args::parse();

    test_php_gen();

    // let openapi = parse_open_api(args.file);
    // testing_command(openapi);
}

fn test_php_gen() {
    let mut class = PHPClass::new(
        "App\\Testing".to_owned(),
        "TestingClass".to_owned(),
    );

    let property = PHPClassProperty::new(
        PHPAccessModifier::PUBLIC,
        "title".to_owned(),
        None,
    );

    let method = PHPFunction::new(
        false,
        "hello".to_owned(),
        vec!["name".to_owned()],
        PHPAccessModifier::PRIVATE,
        vec!["$name = ucfirst($name)".to_owned(), "echo \"Hello $name\"".to_owned()]
    );

    let second_method = PHPFunction::new(
        true,
        "delete".to_owned(),
        vec!["id".to_owned()],
        PHPAccessModifier::PROTECTED,
        vec!["echo $id".to_owned()]
    );

    class.add_import("A\\Really\\Cool\\Library".to_owned());
    class.add_property(property);
    class.add_method(method);
    class.add_method(second_method);

    let data = class.to_string();
    fs::write("output/".to_owned() + &class.get_name() + ".php", data).expect("Unable to write file");
}

fn testing_command(api: OpenAPI) {
    print_paths(api)
}

fn print_paths(api: OpenAPI) {
    for ele in api.paths {
        let route = ele.0;
        let path_item = ele.1.into_item()
            .expect("Unable to gather Path Item");

        println!("\r\nName: {:?}", route);
        println!("Description: {:?}", path_item.description);
        println!("Summary: {:?}", path_item.summary);
        print_parameters(path_item);
    }
}

fn print_parameters(item: PathItem) {
    println!("Parameters");
    for ele in item.parameters {
        let parameter = ele.into_item()
            .expect("Unable to create parameter")
            .parameter_data();

        println!("  name: {:?}", parameter.name);
        println!("  description: {:?}", parameter.description);
        println!("  required: {:?}", parameter.required);
        println!("  deprecated: {:?}", parameter.deprecated);
        println!("  example: {:?}", parameter.example);
        println!("  examples: {:?}", parameter.examples);
        println!("  explode: {:?}", parameter.explode);
        println!("  extensions: {:?}", parameter.extensions);
    }
}

fn parse_open_api(path: String) -> OpenAPI {
    let data = fs::read_to_string(path)
        .expect("Unable to read file");
    
    let openapi: OpenAPI = serde_json::from_str(&data)
        .expect("Could not deserialize input");

    return openapi;
}
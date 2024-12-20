use std::{collections::HashMap, time::Duration};

use clap::{command, Parser};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use surf::http::convert::json;

#[derive(Parser)]
#[command(version, about = "Introspect multiple GraphQL APIs at once")]
#[command(name = "Demo CLI")]
struct Cli {
    /// The path to the config file
    config: std::path::PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    urls: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    println!("file: {:?}", args.config);

    if let Ok(file_str) = std::fs::read_to_string(args.config) {
        if let Ok(yaml) = serde_yml::from_str::<Config>(&file_str) {
            println!("{:?}", yaml);



            let progress_style = ProgressStyle::with_template("{spinner:.yellow} {msg}")
                .unwrap()
                // For more spinners check out the cli-spinners project:
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&[
                    "▰▱▱▱▱▱▱",
                    "▰▰▱▱▱▱▱",
                    "▰▰▰▱▱▱▱",
                    "▰▰▰▰▱▱▱",
                    "▰▰▰▰▰▱▱",
                    "▰▰▰▰▰▰▱",
                    "▰▰▰▰▰▰▰",
                    "▰▱▱▱▱▱▱",
                ]);
            

            for (name, url) in yaml.urls {
                let bar = ProgressBar::new_spinner();
                bar.set_message(format!("Introspecting {name}..." ));
            bar.set_style(progress_style.clone());
            bar.enable_steady_tick(Duration::from_millis(80));
                match surf::post(url)
                    .body(json!({
                                "operationName": "IntrospectionQuery",
                                "query": "query IntrospectionQuery {\n      __schema {\n        \n        queryType { name }\n        mutationType { name }\n        subscriptionType { name }\n        types {\n          ...FullType\n        }\n        directives {\n          name\n          description\n          \n          locations\n          args {\n            ...InputValue\n          }\n        }\n      }\n    }\n\n    fragment FullType on __Type {\n      kind\n      name\n      description\n      \n      fields(includeDeprecated: true) {\n        name\n        description\n        args {\n          ...InputValue\n        }\n        type {\n          ...TypeRef\n        }\n        isDeprecated\n        deprecationReason\n      }\n      inputFields {\n        ...InputValue\n      }\n      interfaces {\n        ...TypeRef\n      }\n      enumValues(includeDeprecated: true) {\n        name\n        description\n        isDeprecated\n        deprecationReason\n      }\n      possibleTypes {\n        ...TypeRef\n      }\n    }\n\n    fragment InputValue on __InputValue {\n      name\n      description\n      type { ...TypeRef }\n      defaultValue\n      \n      \n    }\n\n    fragment TypeRef on __Type {\n      kind\n      name\n      ofType {\n        kind\n        name\n        ofType {\n          kind\n          name\n          ofType {\n            kind\n            name\n            ofType {\n              kind\n              name\n              ofType {\n                kind\n                name\n                ofType {\n                  kind\n                  name\n                  ofType {\n                    kind\n                    name\n                  }\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n"
                    }))
                    .await
                {
                    Ok(mut res) => {
                        let res_1: serde_json::Value = res.body_json().await.unwrap();
                        
                        bar.finish();
                    },
                    Err(_) => todo!(),
                }
            }
        }
    };
}

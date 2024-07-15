extern crate rouille;
extern crate serde;


use rouille::Response;
use serde::Deserialize;
use log::{debug, error, info, log_enabled, warn, Level};
use url::Url as OtherUrl;
use rouille::cgi::CgiRun;
use std::process::Command;
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    address: String,
    port: i32,
    document_root: String,
    bins: Option<HashMap<String, Bin>>
}

#[derive(Deserialize, Debug, Clone)]
struct Bin {
    path: String
}

fn config_filename() -> String {
    match ::std::env::args().nth(1) {
        Some(path) => path,
        None => "mycgi.toml".to_string()
    }
}

fn default_config() -> Config {
    Config {
            address: "localhost".to_string(), 
            port: 8000, 
            document_root: "private_html".to_string(),
            bins: None
        }
}

fn load_config() -> Config {
    match std::fs::read_to_string(config_filename()) {
        Ok(config_str) => {
            toml::from_str(&config_str).unwrap()
        }
        Err(_) => {
            warn!("Could not find or parse config at: {}, using default configuration...", config_filename());
            default_config()
        }
    }
}

fn addr(config: &Config) -> String {
    format!("{}:{}", config.address, config.port.to_string())
}


fn main() {
    env_logger::init();
    
    let config = load_config();
    let addr = addr(&config);

    info!("Starting mycgi on {} with document_root: {}", addr, config.document_root);

    rouille::start_server(addr, move |request| {

        // CGI handling
        {
            if config.bins.is_some() && request.url().starts_with("/cgi") {
                let full_url = "http://example.com".to_string() + &request.url().as_ref(); 
                let parsed_url = OtherUrl::parse(&full_url).unwrap();
                let url_path = parsed_url.path().strip_prefix("/cgi/").unwrap();
                let bin = config.bins.as_ref().unwrap().get(url_path);
                debug!("cgi url_path is {}", url_path);
                if bin.is_some() {
                  
                    let cmd_path= bin.unwrap().path.to_string();
                   
                    debug!("cmd_path {:?}", cmd_path);
                    let doc_root_path = Path::new(config.document_root.as_str());
                    let full_command_path = doc_root_path.join("cgi").join(cmd_path);
                    
                    let mut cmd = Command::new(full_command_path);
                    cmd.env("REDIRECT_STATUS", "1");
                    return cmd.start_cgi(request).unwrap()
                }
            }
        }

        // Static file handling
        {
            let response = rouille::match_assets(request, &config.document_root);
            if response.is_success() {
                return response;
            }
        }

        Response::text("not found").with_status_code(404)
    });
}

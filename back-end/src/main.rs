use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use actix_web_lab::web::spa;
use std::{fs};

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct PrimeRelics {
    name: String,
    is_vaulted: bool,
}

impl PrimeRelics {
    fn new() -> PrimeRelics {
        PrimeRelics { name: "".to_string(), is_vaulted: false }
    }
}

#[derive(Clone, Serialize)]
struct PrimeComponents {
    name: String,
    relics: Vec<PrimeRelics>,
}

impl PrimeComponents {
    fn new() -> PrimeComponents {
        PrimeComponents { name: "".to_string(), relics: vec![PrimeRelics::new(); 0]}   
    }
}

#[derive(Serialize)]
struct PrimeItem {
    name: String,
    components: Vec<PrimeComponents>,
}

impl PrimeItem{
    fn new() -> PrimeItem{
        let componenets: Vec<PrimeComponents> = vec![PrimeComponents::new(); 0];
        PrimeItem { name: "".to_string(), components: componenets }
    }
}

async fn is_relic_vaulted(name: String) -> bool {
    let cached_data_result = fs::read_to_string("./db/relic_data.json");
    
    let cached_data = match cached_data_result {
        Ok(file) => file,
        Err(_) => "nofile".to_string(),
    };

    let create_dir = fs::create_dir_all("./db");

    match create_dir {
        Err(error) => panic!("I/O Error: {:?}", error),
        Ok(_) => (),
    };

    let mut cached_relics: Vec<PrimeRelics> = Vec::new();

    if cached_data != "nofile" {
        cached_relics = serde_json::from_str(cached_data.as_str()).expect("JSON was not well formed...");

        for relic in cached_relics.iter() {
            if relic.name == name {
                return relic.is_vaulted;
            }
        }
    }   

    // If relic was not cached, then hit api
    let client = reqwest::Client::new();
    let url: String = "https://api.warframestat.us/items/search/".to_string();

    // Can add query parameters by appending /?param1=value&param2=value ect...
    let resp = client.get(url + &name.as_str().replace("Relic", "") + "/?only=vaulted").send().await;

    let s: String = resp.unwrap().text().await.unwrap();
    let s_slice: &str = &s[..];
    let datas: Vec<Value> = serde_json::from_str(s_slice).unwrap();

    let api_vaulted_result: Option<bool> = datas[0]["vaulted"].as_bool();

    let api_vaulted: bool = match api_vaulted_result {
        Some(x) => x,
        None => panic!("API didn't recognize relic"),
    };

    let api_relic: PrimeRelics = PrimeRelics { name: name, is_vaulted: api_vaulted };

    cached_relics.push(api_relic);

    let write_success= fs::write("./db/relic_data.json", serde_json::to_string(&cached_relics).unwrap());

    match write_success {
        Err(error) => panic!("I/O Error: {:?}", error),
        Ok(_) => (),
    };

    api_vaulted

}

#[get("/prime-item/{name}")]
async fn search(name: web::Path<String>) -> Result<impl Responder> {

    let url: String = "https://api.warframestat.us/items/search/".to_string();

    let client = reqwest::Client::new();

    // Can add query parameters by appending /?param1=value&param2=value ect...
    let resp = client.get(url + name.as_str()).send().await;

    let s: String = resp.unwrap().text().await.unwrap();
    let s_slice: &str = &s[..];
    let datas: Vec<Value> = serde_json::from_str(s_slice).unwrap();

    let mut response: PrimeItem = PrimeItem::new();

    for val in datas.iter(){
        let val_name: String = val["name"].to_string().replace("\"", "");
        let val_cat: String = val["category"].to_string();
        if val_name.contains("Prime") && 
        (val_cat.to_string().contains("Warframe") ||
        val_cat.to_string().contains("Arch-Gun") ||
        val_cat.to_string().contains("Archwing") ||
        val_cat.to_string().contains("Sentinels") ||
        val_cat.to_string().contains("Primary") ||
        val_cat.to_string().contains("Secondary") ||
        val_cat.to_string().contains("Melee")){
            response.name = val_name;
            for part in val["components"].as_array().unwrap().iter(){
                let mut cur_component: PrimeComponents = PrimeComponents::new();
                let component_name: String = part["name"].to_string().replace("\"", "");
                cur_component.name = component_name;

                for relic in part["drops"].as_array().unwrap().iter(){
                    let mut cur_relic: PrimeRelics = PrimeRelics::new();
                    let relic_name: String = relic["location"].to_string().replace("\"", "");
                    if !(relic_name.ends_with(")")){
                        cur_relic.name = relic_name.clone();
                        cur_relic.is_vaulted = is_relic_vaulted(relic_name).await;
                        cur_component.relics.push(cur_relic);
                    }   
                }
                response.components.push(cur_component)
            }
        }
    }
    
    Ok(web::Json(response))
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(greet)
        .service(search)
        .service(
            spa()
                .index_file("./front-end/build/index.html")
                .static_resources_mount("/static")
                .static_resources_location("./front-end/build/static/")
                .finish(),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    //Add loop here to refresh and store relic info locally
    // https://users.rust-lang.org/t/update-an-actix-state-periodically/50892
}

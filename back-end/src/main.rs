use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;
use serde_json::{Value};
use actix_web_lab::web::spa;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[derive(Clone, Serialize)]
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

#[get("/prime-item/{name}")]
async fn search(name: web::Path<String>) -> Result<impl Responder> {
    // set query with headers
    // refactor with https://stackoverflow.com/questions/47911513/how-do-i-set-the-request-headers-using-reqwest

    let url: String = "https://api.warframestat.us/items/search/".to_string();

    let resp = reqwest::get(url + name.as_str()).await;

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
                        cur_relic.name = relic_name;
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

use actix_web::{get, App, web, HttpServer, Responder, HttpRequest};
use std::collections::BTreeMap;

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let mut map = BTreeMap::new();
    map.insert("x".to_string(), 1.0);
    map.insert("y".to_string(), 2.0);

    // Serialize it to a YAML string.
    let s = serde_yaml::to_string(&map);

    format!("{}", s.unwrap())
}

#[get("/get_config/{name}")]
async fn get_config(path: web::Path<(String)>) -> impl Responder {

    let name = path.into_inner();
    let f = std::fs::File::open("./src/config/site.yaml").unwrap();
    let d: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();
    
    match d["web"].get(name) {
        Some(val) => {format!("{:?}", get_value(val))}
        None => {format!("{:?}", "沒有這個設定變數名稱")}
    }
    
}

fn get_value(arg: &serde_yaml::Value) -> String {

    if arg.is_i64() {
        match arg.as_i64() {
            Some(val) => {return val.to_string()}
            None => {return String::from("0")}
        }
    } 
    
    if arg.is_string() {
        match arg.as_str() {
            Some(val) => {return val.to_string()}
            None => {return String::from("")}
        }
    }

    if arg.is_bool() {
        match arg.as_bool() {
            Some(val) => {return val.to_string()}
            None => {return String::from("false")}
        }
    }

    return String::from("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index).service(get_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
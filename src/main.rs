#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};


#[derive(Serialize)]
struct TemplateContext{
    hoge: String,
    items: Vec<String>,
}

/*
#[get("/kazuha/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
*/

#[get("/test/kazuha")]
#[error(404)]
fn err() -> &'static str{
    "error!"
}

#[get("/test")]
fn test() -> Template{
    //hashmap:pythonの辞典(dict型)
    //let mut context = HashMap::new();
    let hoge= "hogehoge".to_string();
    let mut items = Vec::new();
    for i in 0..10{
       items.push("hoge".to_string());
    }
    let context = TemplateContext{
        hoge: hoge,
        //items: vec!["hoge".to_string(),"hage".to_string()],
        items: items,
    };
    //let mut Number = vec!["hoge", "hage"];
    //context.insert("hoge", hoge);
    //context.insert("number", Number());
    Template::render("test", &context)
    //Template::render("test", &number)
}

//#[get("/<path..>")]
#[get("/assets/<path..>")]
fn extern_open(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/stylesheets/").join(path)).ok()
    //ルートディレクトリからの相対パスで指定。
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![err, test, extern_open]).launch();
}
use std::fs;
use rss::Channel;
use shared::Site;
use std::error::Error;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use std::sync::RwLock;
use std::time::Duration;
use tokio::time::{interval, Interval};
use reqwest::Client;


struct PageData {
  pages: RwLock<Vec<Site>>
}

//#[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
  const IP: (&str, u16) = ("127.0.0.1", 8080);

  tokio::spawn(auto_update_read_pages(IP));

  let data = web::Data::new(PageData {
    pages: RwLock::new(Vec::<Site>::new()),
  });

  HttpServer::new(move || {
    App::new()
      .app_data(data.clone())
      .service(greet)
      .service(update_readers)
      .service(get_readers)
  })
  .bind(IP)?
  .run()
  .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
  format!("Hello {name}!")
}

#[post("/readers")]
async fn update_readers(data: web::Data<PageData>) -> impl Responder {
  let pages = fs::read_to_string("site_list").unwrap();
  let rss_pages:Vec<&str> = pages.lines().collect();

  //let mut sites = Vec::<Site>::new();
  let mut sites = data.pages.write().unwrap();
  sites.clear();

  for page in rss_pages {
    match get_read_pages(&page).await {
      Err(_) => (),
      Ok(mut e) => sites.append(&mut e),
    }
  }
  println!("{}", sites.len());

  HttpResponse::Ok().body("updated")
}

async fn get_read_pages(pages: &str) -> Result<Vec<Site>, Box<dyn Error>> {
  let content = reqwest::get(pages).await?.bytes().await?;
  let channel = Channel::read_from(&content[..])?;

  let mut sites = Vec::<Site>::new();
  for item in channel.items() {
    sites.push(Site::new(item));
  }

  return Ok(sites);
}

#[get("/readers")]
async fn get_readers(data: web::Data<PageData>) -> impl Responder {
  let page_lock = data.pages.read().unwrap();
  let page_clone = page_lock.clone();
  drop(page_lock);

  HttpResponse::Ok().json(page_clone)
}

async fn auto_update_read_pages(ip: (&str, u16)) {
  let mut interval: Interval = interval(Duration::from_secs(3600));
  let client = Client::new();
  let url = format!("http://{}:{}/readers", ip.0, ip.1);
  println!("{}", url);
  loop {
    interval.tick().await;
    let _result = client.post(url.clone()).send().await;
  }
}

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let pages = fs::read_to_string("site_list")?;
  let rss_pages:Vec<&str> = pages.lines().collect();
  
  //let content = File::open("test")?;
  //let channel = Channel::read_from(BufReader::new(content))?;
  let mut sites = Vec::<Site>::new();
  for page in rss_pages {
    match get_readers(&page).await {
      Err(_) => (),
      Ok(mut e) => sites.append(&mut e),
    }
  }

  //dbg!(sites);
  for s in sites {
    println!("{}", s.title);
  }
  Ok(())
}


async fn get_readers(pages: &str) -> Result<Vec<Site>, Box<dyn Error>> {
  let content = reqwest::get(pages).await?.bytes().await?;
  let channel = Channel::read_from(&content[..])?;

  let mut sites = Vec::<Site>::new();
  for item in channel.items() {
    sites.push(Site::new(item));
  }

  return Ok(sites);
}
*/

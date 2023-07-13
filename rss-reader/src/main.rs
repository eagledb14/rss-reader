mod site;

use std::fs;
use site::{Site, parse_xml};
use std::error::Error;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use std::sync::RwLock;
use std::time::Duration;
use tokio::time::{interval, Interval};
use reqwest::Client;


struct PageData {
  pages: RwLock<Vec<Site>>
}

static OPENER: &str = r##"<!DOCKTYPE html><html><script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script><script src="https://unpkg.com/htmx.org@1.9.2" "></script>"##;

static STYLE: &str = r##"<style>
  .entry {
    border-radius: 10px; 
    border: 1px solid #000; 
    padding: 10px; 
    margin-bottom: 30px;
  }
  .center-column {
    height: 200px; 
  }

  @media screen and (min-width: 768px) {
    .center-column {
      width: 50%;
      margin: 0 auto; 
    }
  }

  @media screen and (max-width: 767px) {
    .center-column {
      width: 100%;
    }
  }

  .time {
    font-size: 0.8em; 
    color: #999999; 
  }
</style>
"##;

static CLOSER: &str = r##"</html>"##;

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
      .service(index)
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

#[get("/")]
async fn index(data: web::Data<PageData>) -> impl Responder {
  let page_lock = data.pages.read().unwrap();

  let pages = page(&page_lock, 0, 10).await;
  let body = format!(r##"
  {}
  {}
  <body style="padding: 0; margin: 0;">
  <div class="center-column">
  {}
  </div>
  </body>
  {}"##, OPENER, STYLE, pages, CLOSER);

  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(body)
}

#[post("/r")]
async fn update_readers(data: web::Data<PageData>) -> impl Responder {
  let pages = fs::read_to_string("site_list").unwrap();
  let rss_pages:Vec<&str> = pages.lines().collect();

  let mut sites = data.pages.write().unwrap();
  sites.clear();

  for page in rss_pages {
    match get_read_pages(&page).await {
      Ok(mut e) => sites.append(&mut e),
      Err(e) => println!("{}: {}", e, &page),
    }
  }

  sites.sort_by_key(|item| std::cmp::Reverse(item.date));


  HttpResponse::Ok().body("updated")
}

async fn get_read_pages(pages: &str) -> Result<Vec<Site>, Box<dyn Error>> {
  let content = reqwest::get(pages).await?.bytes().await?;

  return Ok(parse_xml(content.to_vec()))
}

#[get("/r/{page_num}")]
async fn get_readers(data: web::Data<PageData>, page_num: web::Path<usize>) -> impl Responder {
  let page_lock = data.pages.read().unwrap();

  //ok I have no idea what will happen if the sites list is smaller than 10, honeslty paging will
  //be kinda weird
  let page_size = usize::min(10, page_lock.len());
  let num = usize::max(page_size, page_num.into_inner());
  let start = num - page_size;
  
  let response = if start > page_lock.len() {
    "<p>that's it lol, go do something else with your life</p>".to_string()
  }
  else {
    let end = usize::min(num, page_lock.len());
    page(&page_lock, start, end).await
  };

  drop(page_lock);

  response
}

async fn auto_update_read_pages(ip: (&str, u16)) {
  let mut interval: Interval = interval(Duration::from_secs(600));
  let client = Client::new();
  let url = format!("http://{}:{}/r", ip.0, ip.1);

  loop {
    interval.tick().await;
    let result = client.post(url.clone()).send().await;
    println!("{:?}", result);
  }
}

async fn page(sites: &Vec<Site>, start: usize, end: usize) -> String {
  let mut page = "".to_string();

  for i in start..end {
    page = format!("{}\n{}", page, sites[i].html);
  }
  page = format!(r##"{}
  <div hx-get="/r/{}" hx-trigger="revealed" hx-swap="outerHTML" hx-target="this">
    Loading...
  </div>
  "##, page, start + 20);

  page
}


use std::fs;
use rss::Channel;
use shared::Site;
use std::error::Error;


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


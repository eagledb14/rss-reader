use chrono::{Utc, DateTime, FixedOffset};
use xml::reader::{EventReader, XmlEvent};
use std::io::{Cursor, BufReader};


#[derive(Debug, Clone)]
pub struct Site {
  pub html: String,
  pub date: i64
}

impl Site {
  pub fn new(title: String, description: String, link: String, mut date: String, comments: String, origin: String) -> Self {
    let dt = if let Ok(dt) = DateTime::parse_from_rfc2822(&date) {
      dt
    }
    else if let Ok(dt) = DateTime::parse_from_rfc3339(&date) {
      dt
    }
    else {
      Utc::now().with_timezone(&FixedOffset::east_opt(-1).unwrap())
    };

    date = dt.format("%b %d %Y, %H:%M:%S").to_string();

    Self {
      html: Site::create_entry(title, description, link, date, comments, origin),
      date: dt.timestamp_millis(),
    }
  }

  pub fn create_entry(title: String, _description: String, link: String, date: String, comments: String, origin: String) -> String {
    let mut entry = format!(r##"
      <div class="entry">
        <h3>
          <a href="{}">{}</a>
          <span class="time">{}</span>
        </h3>
        <p class="time">{}</p>
    "##, link, title, origin, date);

    if comments != "" {
      entry = format!(r##"{} <a href="{}">Comments</a>"##, entry, comments);
    }
    entry = format!(r##"{}</div>"##, entry);

    return entry;
  }
}

#[derive(Default, Debug)]
struct SiteBuilder {
  title: String, 
  description: String,
  link: String,
  date: String,
  comments: String,
  origin: String
}

impl SiteBuilder {
  pub fn build(self) -> Site {
    Site::new(self.title, self.description, self.link, self.date, self.comments, self.origin)
  }
}

pub fn parse_xml(xml_content: Vec<u8>, url: String) -> Vec<Site> {
  let reader = EventReader::new(BufReader::new(Cursor::new(xml_content)));
  let mut site_list = Vec::<Site>::new();

  let mut entry = "".to_string();
  let mut site_builder = SiteBuilder::default();
  site_builder.origin = url.clone();

  for event in reader {
    match event {
      Ok(XmlEvent::StartElement { name, ..}) => {
        entry = name.to_string().split("}").last().unwrap_or(&"").to_string();
      }
      Ok(XmlEvent::Characters(param)) => {
        update_builder(&entry, param, &mut site_builder);
      }
      Ok(XmlEvent::CData(param)) => {
        update_builder(&entry, param, &mut site_builder);
      }
      Ok(XmlEvent::EndElement { name }) => {
        let name = name.to_string().split("}").last().unwrap_or(&"").to_string();

        //push item to list if it is finished
        if name == "item"  || name == "entry" {
          site_list.push(site_builder.build());
          site_builder = SiteBuilder::default();
          site_builder.origin = url.clone();
        }
      }
      Err(e) => {
        eprintln!("Error: {}", e);
      }
      _ => ()
    }
  }

  return site_list;
}

fn update_builder(entry: &str, param: String, site_builder: &mut SiteBuilder) {
  match entry {
    "title" => site_builder.title = param,
    "link" | "id" => site_builder.link = param,
    "description" | "summary" => site_builder.description = param,
    "pubDate" | "published" | "updated" => site_builder.date = param,
    "comments" => site_builder.comments = param,
    _ => ()
  }
}


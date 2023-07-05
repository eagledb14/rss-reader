use rss::Item;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
  pub html: String,
  pub date: i64
}

impl Site {
  pub fn new(item: &Item) -> Self {
    let title = match item.title() {
      Some(e) => e.to_string(),
      None => "".to_owned()
    };

    let description = match item.description() {
      Some(e) => e.to_string(),
      None => "".to_owned()
    };

    let link = match item.link() {
      Some(e) => e.to_string(),
      None => "".to_owned()
    };

    let date = match item.pub_date() {
      Some(e) => e.to_string(),
      None => "".to_string()
    };

    let comments = match item.comments() {
      Some(e) => e.to_string(),
      None => "".to_string()
    };

    Self {
      html: Site::create_entry(title, description, link, date, comments),
      date: match item.pub_date() {
        Some(e) => {
          match DateTime::parse_from_rfc2822(e) {
            Ok(dt) => dt.timestamp_millis(),
            Err(_) => {
              let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards");
              now.as_millis() as i64
            }
          }
        },
        None => 0,
      },
    }
  }

  pub fn create_entry(title: String, description: String, link: String, date: String, comments: String) -> String {
    let mut entry = format!(r##"
      <div class="entry">
        <h3>
          <a href="{}" target = "_blank">{}</a>
        </h3>
        <p class="time">{}</p>
        <p>{}</p> 
    "##, link, title, date, description);

    if comments != "" {
      entry = format!(r##"{} <a href="{}" target = "_blank">Comments</a>"##, entry, comments);
    }
    entry = format!(r##"{}</div>"##, entry);

    return entry;
  }
}

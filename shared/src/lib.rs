use rss::Item;
use chrono::DateTime;
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Site {
  pub title: String,
  pub description: String,
  pub link: String,
  pub date: i64, 
  pub comments: String
}

impl Site {
  pub fn new(item: &Item) -> Self {
    Self {
      title: match item.title() {
        Some(e) => e.to_string(),
        None => "".to_owned()
      }, 
      description: match item.description() {
        Some(e) => e.to_string(),
        None => "".to_owned()
      },
      link: match item.link() {
        Some(e) => e.to_string(),
        None => "".to_owned()
      },
      date: match item.pub_date() {
        Some(e) => {
          match DateTime::parse_from_rfc2822(e) {
            Ok(dt) => dt.timestamp_millis(),
            Err(_) => {
              0
            }
          }
        },
        None => 0,
      },
      comments: match item.link() {
        Some(e) => e.to_string(),
        None => "".to_owned()
      },
    }
  }
}

use rss::Item;

#[allow(dead_code)]
pub struct Site {
  pub title: String,
  pub description: String,
  pub link: String,
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
      comments: match item.link() {
        Some(e) => e.to_string(),
        None => "".to_owned()
      },
    }
  }
}

pub static OPENER: &str = r##"<!DOCKTYPE html><html><script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script><script src="https://unpkg.com/htmx.org@1.9.2" "></script>"##;

pub static STYLE: &str = r##"<style>
  .entry {
    border-radius: 10px; 
    border: 1px solid #ccc;
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

  .header {
    background-color: #193A9E; 
    color: white; 
    padding: 10px; 
    margin-bottom: 20px; 
    display: flex; 
    justify-content: space-between; 
    align-items: center; 
    font-size: 24px; 
    font-weight: bold; 
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2); 
  }

  .header a {
    color: white; 
    text-decoration: none; 
  }

  .header a:hover {
    text-decoration: underline; 
  }

  body {
    background-color: #222; 
    color: #fff; 
    margin: 0; 
    padding: 20px; 
    font-family: Arial, sans-serif; 
  }

  a {
    color: #fff; 
    text-decoration: none; 
  }

  a:hover {
    text-decoration: underline; 
  }
</style>
"##;

pub static HEADER: &str = r##"<div class="header">
  Header Bar
</div>
"##;

pub static CLOSER: &str = r##"</html>"##;

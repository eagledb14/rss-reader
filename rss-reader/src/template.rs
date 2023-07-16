pub static OPENER: &str = r##"<!DOCKTYPE html><html><script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script><script src="https://unpkg.com/htmx.org@1.9.2" "></script>"##;

pub static STYLE: &str = r##"<style>
  :root {
    --primary-color: #193A9E;
    --background-color: #151519;
    --text-color: #fff;
    --secondary-color: #999999;
    --border-color: #ffffff;
  }
  
  * {
    margin: 0px;
    padding: 0px;
  }

  .entry {
    position: relative;
    border-radius: 10px; 
    border: 1px solid var(--border-color);
    padding: 25px 25px 15px 25px;
    margin-bottom: 25px;
    background-color: var(--background-color);
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

  .sub-heading {
    font-size: 0.8em; 
    color: var(--secondary-color); 
    padding-bottom: 15px;
  }

  .header {
    background-color: var(--primary-color); 
    color: var(--text-color); 
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
    color: var(--text-color); 
    text-decoration: none; 
  }

  .header a:hover {
    text-decoration: underline; 
  }

  body {
    background-color: var(--background-color); 
    color: var(--text-color); 
    margin: 0; 
    padding: 0; 
    font-family: Arial, sans-serif; 
  }

  a {
    color: var(--text-color); 
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

use reqwest;
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;
use std::time::{Duration, Instant};


struct Stopwatch {
    start_time: Instant,
    is_running: bool,
    elapsed: Duration,
}

impl Stopwatch {
    fn new() -> Self {
        Stopwatch {
            start_time: Instant::now(),
            is_running: false,
            elapsed: Duration::default(),
        }
    }

    fn start(&mut self) {
        if !self.is_running {
            self.start_time = Instant::now();
            self.is_running = true;
        }
    }

    fn stop(&mut self) {
        if self.is_running {
            let elapsed = self.start_time.elapsed();
            self.elapsed += elapsed;
            self.is_running = false;
        }
    }

    fn reset(&mut self) {
        self.elapsed = Duration::default();
        self.is_running = false;
    }

    fn get_elapsed(&self) -> Duration {
        if self.is_running {
            self.elapsed + self.start_time.elapsed()
        } else {
            self.elapsed
        }
    }
}

async fn check_link_status(url: &str) -> Result<(), reqwest::Error> {
    let response = reqwest::get(url).await?;
    let status = response.status();

    println!("URL: {}, Status: {}", url, status);

    Ok(())
}

async fn parse_urls(url: &str) -> Result<HashSet<String>, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    let fragment = Html::parse_document(&body);

    let selector = Selector::parse("a[href]:not(.facet_link)").unwrap();
    let mut urls = HashSet::new();

    for link in fragment.select(&selector) {
        if let Some(href) = link.value().attr("href") {
            let resolved_url = resolve_url(url, href);
            urls.insert(resolved_url);
        }
    }

    Ok(urls)
}

fn resolve_url(base: &str, href: &str) -> String {
    if let Ok(base_url) = Url::parse(base) {
        if let Ok(resolved_url) = base_url.join(href) {
            if let Some(fragment) = resolved_url.fragment() {
                //resolved_url.set_fragment(None);
                resolved_url.to_string() 
                //+ "#" + fragment
            } else {
                resolved_url.to_string()
            }
        } else {
            href.to_string()
        }
    } else {
        href.to_string()
    }
}

#[tokio::main]
async fn main() {
    let initial_url = "https://www.karuturirs.com";

    // Create a new stopwatch
    let mut stopwatch = Stopwatch::new();

    // Start the stopwatch
    stopwatch.start();

    let mut visited_urls = HashSet::new();
    visited_urls.insert(initial_url.to_string());

    let mut urls_to_visit = vec![initial_url.to_string()];
    let mut i=0;
    while let Some(url) = urls_to_visit.pop() {
        if let Ok(()) = check_link_status(&url).await {
            if i == 0 {
                if let Ok(urls) = parse_urls(&url).await {
                    for url in urls {
                        if !visited_urls.contains(&url) {
                            visited_urls.insert(url.clone());
                            if url.starts_with(initial_url){
                                urls_to_visit.push(url);
                            }
                            
                        }
                    }
                    i = i+1;
                }
            }
        }
    }

    // Stop the stopwatch
    stopwatch.stop();

    // Print the elapsed time
    println!("Elapsed time: {:?}, {}", stopwatch.get_elapsed(), i);

}

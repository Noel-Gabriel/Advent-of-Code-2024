use reqwest::{
    header::{HeaderMap, HeaderValue, COOKIE},
    Client 
};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    env,
};
use dotenv;

pub async fn fetch_html(day: u32, year: i32, root: &Path) {
    let prefix = if day < 10 { "0" } else { "" };
    let target_dir = format!("day{prefix}{day}/src/input{prefix}{day}.txt");
    let target = root.join(Path::new(&target_dir));

    let source = format!("https://adventofcode.com/{year}/day/{day}/input");

    parse(&source, &target).await;
}

async fn parse(url: &String, dest: &PathBuf) {
    let client = Client::new();

    dotenv::dotenv().ok();

    let mut headers = HeaderMap::new();
    let cookie = env::var("AOC2024COOKIE").expect("Cookie not found for AoC 2024 in env.");
    headers.insert(COOKIE, HeaderValue::from_str(&format!("session={cookie}")[..]).unwrap());

    let response = client.get(url)
        .headers(headers)
        .send().await
        .expect("Could not get response from {url}.");

    let html = response
        .text().await
        .expect(&format!("Could not parse response from {url}.")[..]);

    let mut file = File::create(dest)
        .expect(&format!("Could not create file in {}.", dest.display())[..]);

    file
        .write_all(html.as_bytes())
        .expect(&format!("Could not write to file in {}.", dest.display())[..]);
}

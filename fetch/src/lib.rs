use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::env; 
use dotenv;

pub async fn fetch_html(day: i32, root: &String) {
    let target_dir = format!("/day{day}/src/input{day}.txt");
    let target = root.to_owned() + &target_dir[..];

    let source = format!("https://adventofcode.com/2024/day/{day}/input");

    parse(&source, &target).await;
}

async fn parse(url: &String, dest: &String) {
    let client = Client::new();

    dotenv::dotenv().ok();

    // create new session using my cookie value (need to be logged in to see content)
    // need to be changed if cookies are cleared. Maybe pass as an argument?
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
        .expect(&format!("Could not create file in {dest}.")[..]);

    file
        .write_all(html.as_bytes())
        .expect(&format!("Could not write to file in {dest}.")[..]);
}

use std::{
    env,
    path::Path,
    fs,
    process::{Command, exit},
};
use chrono::Datelike;

#[tokio::main]
async fn main()  {
    match verify_args() {
        Ok((day, year))    => {
            let path = format!("/Users/noel/Desktop/Advent-of-Code-{year}");
            let root = Path::new(&path);
            create_and_init(day, &root);
            fetch::fetch_html(day, year, &root).await;
        }
        Err(error) => eprintln!("{error}"),
    }
}

fn verify_args() -> Result<(u32, i32), String> {
    let args: Vec<String> =  env::args().collect();

    let date = chrono::Local::now();
    let current_year = date.year();
    let current_month = date.month();
    let current_day = date.day();

    let err_res = Err(format!("Usage: cargo run <day> <year> or without any arguments to fetch today's puzzle."));

    if args.len() == 1 {
        if current_month != 12 { return Err("It's not december, no puzzle today!".to_string()) }
        return Ok((current_day, current_year))
    } else if args.len() == 3 {
        match (args[1].parse::<u32>(), args[2].parse::<i32>()) {
            (Ok(day_in), Ok(year_in)) => {
                if day_in > 25    { return Err("Day needs to be between 1 and 25.".to_string()) }
                if year_in < 2015 { return Err("First AoC was in 2015.".to_string()) }
                if current_year < year_in || current_year == year_in && day_in > current_day { 
                    return Err("Invalid year and day, can't fetch future AoC.".to_string()) 
                }
                return Ok((day_in, year_in))
            }
            _ => return err_res 
        }
    }
    err_res
}

fn create_and_init(day: u32, root: &Path) {
    // only creates sub directories for each day, not the git repo
    if !root.exists() { 
        eprintln!("{} does not exist!.", root.display()); 
        exit(1); 
    }

    let prefix = if day < 10 { "0" } else { "" };
    let dir = format!("day{prefix}{day}").to_string();
    let path_dir = root.join(Path::new(&dir));

    if let Err(err) = fs::create_dir(&path_dir) {
        eprintln!("{err}");
        exit(1);
    }

    if let Err(err) = Command::new("cargo")
        .arg("init")
        .arg(&path_dir)
        .output() {
            eprintln!("{err}");
            eprintln!("Dir: {}", path_dir.display());
            exit(1);
    }
}

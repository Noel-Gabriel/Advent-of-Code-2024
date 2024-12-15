use std::{
    env,
    path::Path,
    fs,
    process::{Command, exit},
};

#[tokio::main]
async fn main()  {
    match verify_args() {
        Ok(day)    => {
            let root = String::from("/Users/noel/Desktop/Advent-of-Code-2024");
            create_and_init(day, &root);
            fetch::fetch_html(day, &root).await;
        }
        Err(error) => eprintln!("{error}"),
    }
}

fn verify_args() -> Result<i32, String> {
    let args: Vec<String> =  env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return Err("Aborting...".to_string());
    }

    match args[1].parse::<i32>() {
        Ok(d) => Ok(d),
        Err(_) => Err("Day needs to be an integer!".to_string()),
    }
}

fn create_and_init(day: i32, root: &String) {
    let dir = format!("/day{day}").to_string();
    let path = root.to_owned() + &dir;

    let path_struct = Path::new(&path[..]);

    if let Err(err) = fs::create_dir(path_struct) {
        eprintln!("{err}");
        exit(1);
    }

    if let Err(err) = Command::new("cargo")
        .arg("init")
        .arg(&path)
        .output() {
            eprintln!("{err}");
            eprintln!("Dir: {path}");
            exit(1);
    }
}

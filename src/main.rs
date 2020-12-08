mod parse_arguments;
mod global_path;
mod message;
mod interactive_input;
mod app;
mod scraping;
mod selector;

use parse_arguments::parse_args;
use global_path::{DOWNLOAD_DIR, INDEX_HTML};
use message::{TITLE};
use interactive_input::{
    select_lang::select_lang, 
    select_bound::select_lower_bound, 
    select_bound::select_upper_bound,
    select_topic::select_topic
};
use app::App;
use scraping::scraping_github;

use anyhow::Result;
use console::style;
use chrono::Local;

use std::process;
use std::fs::{self, File};
use std::io::Write;
use std::thread;
use std::time::Duration;

static _HTML_DATA: &[u8] = include_bytes!("./theme/index.html");

fn exit<T>(error_message: T)
where
    T: std::fmt::Display
{
    eprintln!("{} {}", style("Error:").red(), error_message);
    process::exit(1);
}

fn setup_environment() -> Result<()> {
    if !DOWNLOAD_DIR.exists() {
        fs::create_dir(DOWNLOAD_DIR.as_path())?;
    }

    if !INDEX_HTML.exists() {
        let mut index_html = File::create(INDEX_HTML.as_path())?;
        index_html.write_all(_HTML_DATA)?;
    }    

    Ok(())
}

fn check_json_file() -> u64 {
    let mut json_file_count = 1;
    for dir_entry in DOWNLOAD_DIR.read_dir().expect("cannot read dir") {
        if let Ok(entry) = dir_entry {
            let entry = entry.path().as_path().to_str().unwrap().to_string();
            if entry.ends_with(".json") && entry.contains("No-") && entry.contains("-lang_") && entry.contains("-range_") && entry.contains("-topics_") {
                json_file_count += 1;
            }
        }
    }

    json_file_count
}

fn create_json_file(app: &App, json_file_count: u64) -> Result<()> {
    let now = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let topic = match app.topics() {
        "" => "None",
        _ => app.topics()
    };

    let json_path = format!("No-{No}_{Lang}-lang_{Low}-{Up}-range_{Topic}-topics_{Time}.json",
            No    = json_file_count,
            Lang  = app.search_lang(),
            Low   = app.lower_bound(),
            Up    = app.upper_bound(),
            Topic = topic.to_string().replace(" ", "-"),
            Time  = now
    );

    let mut abs_json_path = DOWNLOAD_DIR.clone();
    abs_json_path.push(json_path);

    let serialize = serde_json::to_vec_pretty(&app.contents)?;
    let mut file = File::create(&abs_json_path)?;
    file.write_all(&serialize)?;

    Ok(())
}

fn open_index_html() {
    if cfg!(target_os = "macos") {
        process::Command::new("open")
            .arg(INDEX_HTML.as_path())
            .spawn()
            .unwrap_or_else(|_| panic!("cannot open {}", INDEX_HTML.as_path().to_str().unwrap()));
    } else {
        println!("\n{} open {}", style("info:").green(), INDEX_HTML.as_path().to_str().unwrap());
    }
}

fn main() {
    // `git-spy` needs no arguments
    let _ = parse_args();
    
    setup_environment().expect("setup failed...");

    if let Err(e) = headless_chrome::browser::default_executable() {
        exit(&e);
    }

    println!("{}", style(TITLE).green());

    let search_lang: String = select_lang();
    let lower_bound: String = select_lower_bound();
    let upper_bound: String = select_upper_bound();

    if upper_bound.as_str() != "*" {
        let low: u64 = lower_bound.parse().unwrap();
        let up: u64 = upper_bound.parse().unwrap();
        if low >= up {
            exit("incorrect range queries. [ lower < upper ]");
        }
    }

    let topics: String = select_topic();

    let mut app = App::new(&search_lang, &lower_bound, &upper_bound, &topics);
    app.push_queries();

    println!("{} {}", style("headless mode:").green(), app.headless_mode());
    println!("{} {}", style("search query: ").green(), app.queries());

    if let Err(e) = scraping_github(&mut app) {
        exit(&e);
    }

    let json_file_count = check_json_file();
    create_json_file(&app, json_file_count).expect("cannot create json file");

    thread::sleep(Duration::from_secs(3));
    open_index_html();
}
use crate::selector::{
    DESC_SELECTOR_ARRAY, 
    GITHUB_INPUT_SELECTOR, 
    GITHUB_URL, 
    HIT_REPOSITORY_SELECTOR, 
    KEYWORDS_SELECTOR_ARRAY, 
    LENGTH_OF_ELEMENTS, 
    NEXT_PAGE_BUTTON_SELECTOR, 
    REPO_NAME_AND_HREF_SELECTOR_ARRAY, 
    STAR_SELECTOR_ARRAY
};
use crate::app::JsonData;
use crate::app::App;

use anyhow::Result;
use console::style;
use serde_json::value;
use headless_chrome::{
    Browser, 
    LaunchOptionsBuilder,
};

use std::thread;
use std::time::Duration;

pub fn scraping_github(app: &mut App) -> Result<(), failure::Error> {

    let browser = Browser::new(LaunchOptionsBuilder::default()
        .headless(app.headless_mode())
        .window_size(Some((1600, 800)))
        .build().unwrap())?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(GITHUB_URL)?;
    //wait for input tag is shown
    tab.wait_for_element(GITHUB_INPUT_SELECTOR)?.click()?;

    thread::sleep(Duration::from_secs(7));

    //search repo
    tab.type_str(&app.queries())?.press_key("Enter")?;

    if let Err(e) = tab.wait_for_element(HIT_REPOSITORY_SELECTOR) {
        eprintln!("\ncannot find repository... match 0");
        return Err(e);
    }

    let hit_repo = match tab
        .wait_for_element(HIT_REPOSITORY_SELECTOR)?
        .call_js_fn("function() { return this.innerText; }", true)?
        .value.unwrap() {
            value::Value::String(s) => s,
            _ => "unknown".to_string()
        };
    println!("\n{} {} (The maximum number of repositories git-spy can get is 100)", style("info:").green() , hit_repo);

    let mut json_contents: Vec<JsonData> = Vec::new();
    
    loop {
        tab.wait_for_element(HIT_REPOSITORY_SELECTOR)?;
        thread::sleep(Duration::from_secs(5));

        let target_element = tab.wait_for_elements(LENGTH_OF_ELEMENTS)?;
        println!("{} hit {} repositories", style("info:").green(), target_element.len());

        app.add_web_page_count();
        app.add_repository_count(target_element.len() as u64);

        for i in 0..target_element.len() {

            let url = match tab
                .wait_for_element(REPO_NAME_AND_HREF_SELECTOR_ARRAY[i])?
                .call_js_fn("function() { return this.href }", true)?
                .value.unwrap() {
                    value::Value::String(s) => s,
                    _ => "NOT_FOUND".to_string()
                };
            println!("{} {}", style("get: ").magenta(), url);

            let repo = match tab
                .wait_for_element(REPO_NAME_AND_HREF_SELECTOR_ARRAY[i])?
                .call_js_fn("function() { return this.innerText }", true)?
                .value.unwrap() {
                    value::Value::String(mut s) => {

                        let mut repository_name = String::new();
                        while let Some(ch) = s.pop() {
                            if ch == '/' {
                                break;
                            }
                            repository_name.push(ch);
                        }

                        repository_name.chars().rev().collect::<String>()

                    },
                    _ => "NOT_FOUND".to_string()
                };

            let desc = if tab.wait_for_element(DESC_SELECTOR_ARRAY[i]).is_err() {
                String::from("NOT_FOUND")
            } else {
                match tab
                    .wait_for_element(DESC_SELECTOR_ARRAY[i])?
                    .call_js_fn("function() { return this.innerText }", true)?
                    .value.unwrap() {
                        value::Value::String(s) => s,
                        _ => "NOT_FOUND".to_string()
                    }
            };

            let star = if tab.wait_for_element(STAR_SELECTOR_ARRAY[i]).is_err() {
                String::from("NOT_FOUND")
            } else {
                match tab
                    .wait_for_element(STAR_SELECTOR_ARRAY[i])?
                    .call_js_fn("function() { return this.innerText }", true)?
                    .value.unwrap() {
                        value::Value::String(s) => s.trim_start().to_string(),
                        _ => "NOT_FOUND".to_string()
                   }
            };

            let keywords = if tab.wait_for_element(KEYWORDS_SELECTOR_ARRAY[i]).is_err() {
                String::from("NOT_FOUND")
            } else {
                match tab
                    .wait_for_element(KEYWORDS_SELECTOR_ARRAY[i])?
                    .call_js_fn("function() { return this.innerText }", true)?
                    .value.unwrap() {
                        value::Value::String(mut s) => {
                            if s.contains("license") || s.contains("ago") {
                                s.clear();
                                s.push_str("NOT_FOUND");
                            }
        
                            s
                        },
                        _ => "NOT_FOUND".to_string()
                    }
            };

            json_contents.push(JsonData::new(&url, &repo, &desc, &star, &keywords));
        } //for end

        if tab.wait_for_element(NEXT_PAGE_BUTTON_SELECTOR).is_err() {

            println!("\n{} {} repositories acquired. ($HOME/Downloads/git-spy-result/xxxxx.json)", style("info:").green(), app.total_repository_count());

            app.contents = json_contents;
            println!("{} scraping successfully finished.", style("info:").green());
            break;

        } else if app.web_page_count() >= 10 {

            println!("\n{} {} repositories acquired. ($HOME/Downloads/git-spy-result/xxxxx.json)", style("info:").green(), app.total_repository_count());

            app.contents = json_contents;
            println!("{} The maximum number of items(100) has been reached.", style("info:").green());
            break;
        }
        
        println!("{} go to the next page\n", style("info:").green());
        tab.wait_for_element(NEXT_PAGE_BUTTON_SELECTOR)?.click()?;
    } //loop end
    
    Ok(())
}
use console::style;
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct JsonData {
    url: String,
    repo: String,
    desc: String,
    star: String,
    keywords: String
}

impl JsonData {
    pub fn new(url: &str, repo: &str, desc: &str, star: &str, keywords: &str) -> Self {
        Self {
            url: url.to_string(),
            repo: repo.to_string(),
            desc: desc.to_string(),
            star: star.to_string(),
            keywords: keywords.to_string()
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    search_lang: String,
    lower_bound: String,
    upper_bound: String,
    topics: String,
    headless_mode: bool,
    queries: String,
    web_page_count: u64,
    total_repository_count: u64,
    contents: Box<Vec<JsonData>>
}

impl App {
    pub fn new(lang: &str, lower: &str, upper: &str, topics: &str, headless: bool) -> Self {
        Self {
            search_lang: lang.to_string(),
            lower_bound: lower.to_string(),
            upper_bound: upper.to_string(),
            topics: topics.to_string(),
            headless_mode: headless,
            ..App::default()
        }
    }

    pub fn search_lang(&self) -> &str {
        &self.search_lang
    }

    pub fn lower_bound(&self) -> &str {
        &self.lower_bound
    }

    pub fn upper_bound(&self) -> &str {
        &self.upper_bound
    }

    pub fn topics(&self) -> &str {
        &self.topics
    }

    pub fn headless_mode(&self) -> bool {
        self.headless_mode
    }

    pub fn queries(&self) -> &str {
        &self.queries
    }

    pub fn web_page_count(&self) -> u64 {
        self.web_page_count
    }

    pub fn total_repository_count(&self) -> u64 {
        self.total_repository_count
    }

    pub fn json_contents(&self) -> &Box<Vec<JsonData>> {
        &self.contents
    }

    pub fn add_web_page_count(&mut self) {
        self.web_page_count += 1;
        println!("{} page {}/10", style("info:").green(), self.web_page_count);
    }

    pub fn add_repository_count(&mut self, repository_count: u64)  {
        self.total_repository_count += repository_count;
    }

    pub fn set_json_contents(&mut self, json: Box<Vec<JsonData>>) {
        self.contents = json;
    }

    pub fn push_queries(&mut self) {
        if self.topics == "".to_string() {
            self.queries = format!("language:{} stars:{}..{}", self.search_lang, self.lower_bound, self.upper_bound);
        } else {
            self.queries = match self.topics.split(' ').collect::<Vec<&str>>().len() {
                1 => format!("language:{} stars:{}..{} topic:{}", self.search_lang, self.lower_bound, self.upper_bound, self.topics),
                _ => format!("language:{} stars:{}..{} topic:\"{}\"", self.search_lang, self.lower_bound, self.upper_bound, self.topics)
            }
        }
    }
}
# git-spy 0.1.2

## changed the data structure of the contents of the app structure

`contents: Box<Vec<JsonData>>` -> `pub contnets: Vec<JsonData>`

##### **`src/app.rs`**
```rust
#[derive(Debug, Default)]
pub struct App {
    ...
    ...
    pub contents: Vec<JsonData>
}
```

## iterator was used to refactor

extracts only the github repository name from the url string

##### **`src/scraping.rs`**
```rust
let mut repository_name = String::new();
while let Some(ch) = s.pop() {
    if ch == '/' {
        break;
    }
    repository_name.push(ch);
}

repository_name.chars().rev().collect::<String>()
```

# git-spy 0.1.1

## added conditions of language selection

If you press the Enter key without selecting a language, you will be prompted to select a language again.

##### **`src/interactive_input/select_lang.rs`**
```rust
if s == "" {
    eprintln!("{} input language...", style("info:").red());
    continue;
}
```

## modified open_index_html function (src/main.rs)

Removed arguments.

# git-spy 0.1.0

initial release
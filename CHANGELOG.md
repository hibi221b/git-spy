# git-spy 0.1.1

## Added conditions of language selection

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
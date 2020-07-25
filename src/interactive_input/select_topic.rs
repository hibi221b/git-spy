use console::style;

pub fn select_topic() -> String {

    println!("{}", style("▼ topic words.(cli, web, gui, terminal, etc...) If you don't need, press enter.").cyan());

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim().to_string()
}
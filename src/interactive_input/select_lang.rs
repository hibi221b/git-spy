use console::style;

pub fn select_lang() -> String {
    
    println!("{}", style("▼ language").cyan());

    let search_lang = loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        let s = s.trim();
        
        if s == "" {
            eprintln!("{} input language...", style("info:").red());
            continue;
        }

        if s.contains(" ") {
            eprintln!("{} input one language.", style("info:").red());
            continue;
        }
        
        break s.to_string();
    };

    search_lang
}
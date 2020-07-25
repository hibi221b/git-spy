use console::style;

pub fn select_headless_mode() -> bool {

    println!("{}", style("▼ press enter: hide chrome, input `n`: visualize chrome").cyan());

    let headless = loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        let result = match s.trim() {
            "n" => false,
            "" => true,
            _ => {
                eprintln!("{} press Enter/Space or n", style("info:").red());
                continue;
            }
        };

        break result;
    };

    headless
}
use console::style;

pub fn select_lower_bound() -> String {

    println!("{}", style("\u{25bc} lower bound of github star.").cyan());

    let lower = loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
    
        let num: u64 = match s.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("{} input number...", style("info:").red());
                continue;
            }
        };

        if num < 1 {
            eprintln!("{} you need to input the number more than 1", style("info:").red());
            continue;
        }

        break num.to_string();
    };

    lower
}

pub fn select_upper_bound() -> String {

    println!("{}", style("\u{25bc} upper bound of github star. (number or *)").cyan());

    let upper = loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        let s = s.trim();
        if s == "*" {
            break s.to_string();
        } else {
            let num: u64 = match s.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("{} input number or *", style("info:").red());
                    continue;
                }
            };

            if num < 1 {
                eprintln!("{} you need to input the number more than 1 or *", style("info:").red());
                continue;
            }

            break num.to_string();
        }
    };

    upper
}

pub fn report_info(str: &String) {
    let mut f_str = str.clone();
    if f_str.is_empty() {
        f_str = "none".to_string();
    }
    println!("({}): {}", colored::Colorize::blue("Info"), f_str);
}

pub fn report_warn(str: &String) {
    let mut f_str = str.clone();
    if f_str.is_empty() {
        f_str = "none".to_string();
    }
    println!("({}): {}", colored::Colorize::yellow("Warn"), f_str);
}


pub fn report_err(str: &String) {
    let mut f_str = str.clone();
    if f_str.is_empty() {
        f_str = "none".to_string();
    }
    println!("({}): {}", colored::Colorize::red("Error"), f_str);
}

pub fn start_logs(cli: &crate::cli::Cli) {
    
    if cli.quiet {
        report_warn(&"Extra information is not being logged".to_string());
    }

    if !cli.quiet {
        report_info(&cli.file);
        report_info(&cli.identifier);
    }
}

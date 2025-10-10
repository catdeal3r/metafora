use clap::Parser;

pub mod net;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Hide debug information
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
    
    // File to be uploaded
    #[arg(short, long, default_value = "none")]
    file: String,

    // Url to download from
    #[arg(short, long, default_value = "none")]
    url: String,
}


fn main() {
    let cli = Cli::parse();
    let verbose = &!cli.quiet;

    if cli.file != "none" && cli.url != "none" {
        println!("You cannot download and upload a file at the same time.");
        return
    }

    if cli.file != "none" {
        let mut raw_output = String::new();
        
        let result = net::upload_file_and_add_result_to_str(&verbose, &cli.file, &mut raw_output);
        
        net::report_error(result.clone());

        if let Some(output) = raw_output.lines().next() {
            println!("{output}");
        }
        
    } else if cli.url != "none" {
        let mut raw_output = String::new();
        
        let result = net::download_and_add_data_to_str(&verbose, &cli.url, &mut raw_output);
        
        net::report_error(result.clone());

        if let Some(output) = raw_output.lines().next() {
            println!("{output}");
        }
    }
}

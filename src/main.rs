use clap::{Parser, ArgGroup};

pub mod net;
pub mod log;

#[derive(Parser)]
#[command(version, about, long_about)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["file","url"])
))]
pub struct Cli {
    // File to be uploaded
    #[arg(short, long, default_value = "")]
    file: String,

    // Url to download from
    #[arg(short, long, default_value = "")]
    url: String,

    /// Hide debug information
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();
    let verbose = &!cli.quiet;

    log::start_logs(&cli);
    
    if !cli.file.is_empty() {
        let mut raw_output = String::new();
        
        let result = net::upload_file_and_add_result_to_str(&verbose, &cli.file, &mut raw_output);
        
        net::report_error(result.clone());

        if let Some(output) = raw_output.lines().next() {
            println!("{output}");
        }
        
    } else if !cli.url.is_empty() {
        let mut raw_output = String::new();
        
        let result = net::download_and_add_data_to_str(&verbose, &cli.url, &mut raw_output);
        
        net::report_error(result.clone());

        if let Some(output) = raw_output.lines().next() {
            println!("{output}");
        }
    }
}

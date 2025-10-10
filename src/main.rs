use clap::Parser;

pub mod net;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Show debug information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    
    // File to be uploaded
    #[arg(short, long, default_value = "none")]
    file: String,

    // Url to download from
    #[arg(short, long, default_value = "none")]
    url: String,
}


fn main() {
    let cli = Cli::parse();

    if cli.file != "none" && cli.url != "none" {
        println!("You cannot download and upload a file at the same time.");
        return
    }

    if cli.file != "none" {
        let mut raw_output = String::new();
        
        match net::upload_file_and_add_result_to_str(&cli.verbose, &cli.file, &mut raw_output) {
            Err(error) => {
                println!("{}", error.to_string());
                return
            },
            _other  => {},
        };

        if let Some(output) = raw_output.lines().next() {
            println!("{output}");
        }
        
    } else if cli.url != "none" {
        let mut raw_output = String::new();
        
        match net::download_and_add_data_to_str(&cli.verbose, &cli.url, &mut raw_output) {
            Err(error) => {
                println!("{}", error.to_string());
                return
            },
            _other => {},
        };

        if let Some(output) = raw_output.lines().next() {
            println!("{output}");
        }
    }
}

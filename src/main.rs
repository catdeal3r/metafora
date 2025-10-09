use clap::Parser;

pub mod net;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Show debug information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    
    // File to be uploaded
    #[arg(short, long)]
    file: String,
}


fn main() {
    let cli = Cli::parse();
    
    let output = net::upload_file_and_return_result(&cli.verbose, &cli.file).unwrap();
    println!("{output}");
}

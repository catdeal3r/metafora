use clap::Parser;

pub mod net;
pub mod log;
pub mod fs;
pub mod cli;

fn main() {
    let cli = cli::Cli::parse();
    let verbose = &!cli.quiet;

    log::start_logs(&cli);
    
    if !cli.file.is_empty() {
        let mut raw_output = String::new();
        
        let result = net::upload_file_and_add_result_to_str(&verbose, &cli.file, &mut raw_output);
        
        net::report_error(result.clone());

        println!("{raw_output}");
        
    } else if !cli.url.is_empty() {
        let mut raw_output = String::new();
        
        let result = net::download_and_add_data_to_str(&verbose, &cli.url, &mut raw_output);
        
        net::report_error(result.clone());

        fs::create_file_with_content(&raw_output, &cli.output_file_name).unwrap();

        let mut log_str = "File created: ".to_string();
        log_str.push_str(&cli.output_file_name);
        
        log::report_info(&log_str);
    }
}

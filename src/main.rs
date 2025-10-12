use clap::Parser;

pub mod net;
pub mod log;
pub mod fs;
pub mod cli;
pub mod crp;

fn main() {
    let cli = cli::Cli::parse();
    let verbose = &!cli.quiet;

    log::start_logs(&cli);
    
    if !cli.file.is_empty() {
        if !std::path::Path::exists(std::path::Path::new(&cli.file)) {

            let mut log_str = "Can't find file: ".to_string();
            log_str.push_str(&cli.file);

            log::report_err(&log_str);
            return
        }

        let file_contents = std::fs::read_to_string(&cli.file).unwrap();
        let mut encryption_key = String::new();

        let encrypted_bytes = crp::encrypt_str(&file_contents, &mut encryption_key);
        
        let mut raw_output = String::new();
        let result = net::upload_file_and_add_result_to_str(&verbose, &encrypted_bytes, &mut raw_output);
        
        net::report_error(result.clone());

        if *verbose {
            println!("\n---")
        }

        let mut stripped_url = net::clean_upload_url(&raw_output).to_string();

        crp::encode_to_base64(&mut stripped_url);

        let mut log_str = "You can now download this file with this command:\nmetafora -i ".to_string();

        log_str.push_str(&stripped_url);

        log_str.push_str(" -o ");
        log_str.push_str(&cli.file);

        log_str.push_str(" -e ");
        log_str.push_str(&encryption_key);

        if cli.quiet {
            log_str.push_str(" -q");
        }
        
        log::report_info(&log_str);
        
    } else if !cli.identifier.is_empty() {
        
        if cli.encryption_key.is_empty() {
            log::report_warn(&"Not decrypting possibly encrypted file".to_string());
        }

        let mut raw_output: Vec<u8> = Vec::new();

        let mut url = "https://0x0.st/s/".to_string();
        let mut identifier_copy = cli.identifier.clone();

        crp::decode_from_base64(&mut identifier_copy);

        url.push_str(&identifier_copy);

        let result = net::download_and_add_data_to_str(&verbose, &url, &mut raw_output);
        
        net::report_error(result.clone());

        if *verbose {
            println!("\n---")
        }

        let raw_output_str = String::from_utf8_lossy(raw_output.as_slice());

        if raw_output_str.contains("stop reason = invalid address") {
            log::report_err(&"Identifier is invalid, has expired, or has been deleted".to_string());
            return
        }

        if !cli.encryption_key.is_empty() {
            crp::decrypt_str(&mut raw_output, &cli.encryption_key);
        }

        fs::create_file_with_content(&raw_output, &cli.output_file_name).unwrap();

        let mut log_str = "File created: ".to_string();
        log_str.push_str(&cli.output_file_name);
        
        log::report_info(&log_str);
    }
}

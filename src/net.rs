use curl::easy::{Easy, List, Form};

use colored::Colorize;

#[derive(thiserror::Error)]
#[derive(Debug)]
#[derive(Clone)]
pub enum CustomNetError {
  #[error("{}", .0.description())]
  CurlError(#[from] curl::Error),
  
  #[error(transparent)]
  FormError(#[from] curl::FormError),
}

pub fn upload_file_and_add_result_to_str(verbose: &bool, filename: &String, r_output: &mut String) -> Result<(), CustomNetError> {
    let mut request = Easy::new();

    request.url("https://0x0.st/")?;

    let mut headers = List::new();
    headers.append("User-Agent: \"gh:catdealer/metafora/0.1.2\"")?;

    request.http_headers(headers)?;

    if *verbose == true {
        request.verbose(true)?;
    }

    let mut form = Form::new();

    form.part("file")
        .file(filename)
        .add()?;

    form.part("secret")
        .contents("a".as_bytes())
        .add()?;

    request.httppost(form)?;

    let mut transfer = request.transfer();
    transfer.write_function(|data| {
        *r_output = String::from_utf8_lossy(data).to_string();
        Ok(data.len())
    })?;

    transfer.perform()?;

    Ok(())
}

pub fn download_and_add_data_to_str(verbose: &bool, url: &String, r_output: &mut String) -> Result<(), CustomNetError> {
    let mut request = Easy::new();

    request.url(url)?;

    let mut headers = List::new();
    headers.append("User-Agent: \"gh:catdeal3r/metafora/0.1.0\"")?;

    request.http_headers(headers)?;

    if *verbose == true {
        request.verbose(true)?;
    }

    let mut transfer = request.transfer();
    transfer.write_function(|data| {
        *r_output = String::from_utf8_lossy(data).to_string();
        Ok(data.len())
    })?;

    transfer.perform()?;

    Ok(())
}

pub fn report_error(result: Result<(), CustomNetError>) {
    match result.inspect_err(|err| eprintln!("({}): {}", "Error".red(), err)) {
        Err(_) => std::process::exit(1),
        _ => {}
    }
}

pub fn clean_upload_url(str: &str) -> &str {
    let n = 17;
    let mut char_indices = str.char_indices();

    if let Some((byte_index, _)) = char_indices.nth(n) {
        &str[byte_index..].trim_end()
    } else {
        ""
    }
}

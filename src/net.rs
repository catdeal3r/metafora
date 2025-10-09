use curl::easy::{Easy, List, Form};

use std::io::{stdout, Write};
use std::error::Error;

pub fn upload_file_and_return_result(verbose: &bool, filename: &String) -> Result<String, Box<dyn Error>> {
    let mut request = Easy::new();

    request.url("https://0x0.st/")?;

    let mut headers = List::new();
    headers.append("User-Agent: \"darkprism/0.1.0\"")?;

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

    request.write_function(|data| {
        stdout().write_all(data).expect("stdout().write_all() failed.");
        Ok(data.len())
    })?;

    request.perform()?;

    Ok(request.response_code()?.to_string())
}

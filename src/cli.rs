use clap::{Parser, ArgGroup};

#[derive(Parser)]
#[command(version, about, long_about)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["file","url"])
))]
pub struct Cli {
    #[arg(short, long, default_value = "", help = "File to upload", hide_default_value = true)]
    pub file: String,

    #[arg(short, long, default_value = "", requires = "output_file_name", help = "Url to download file from", hide_default_value = true)]
    pub url: String,

    #[arg(short = 'o', long, default_value = "", requires = "url", help = "The filename for a downloaded file", hide_default_value = true)]
    pub output_file_name: String,

    #[arg(short, long, default_value_t = false, help = "Disable verbose output")]
    pub quiet: bool,
}

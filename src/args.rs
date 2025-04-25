use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct BaseCliArgs {}

#[derive(Parser, Debug)]
pub struct RunCmdArgs {
    #[arg(short, long, default_value = "models/")]
    model_path: String,
}

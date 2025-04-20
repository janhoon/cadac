use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CadacCliArgs {
    #[arg(short, long, default_value = "models/")]
    model_path: String,
}

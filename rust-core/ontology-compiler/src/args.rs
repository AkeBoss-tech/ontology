use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory containing .ttl files
    #[arg(short, long, default_value = "ontology-definitions")]
    pub input: PathBuf,

    /// Sidecar YAML file for actions and functions
    #[arg(short, long)]
    pub sidecar: Option<PathBuf>,

    /// Output JSON file
    #[arg(short, long, default_value = "ontology.json")]
    pub output: PathBuf,
}

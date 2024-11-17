use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Storage {
    // Store the data in memory
    #[default]
    Memory,

    // Store the data on disk
    Disk,

    // Store the data in a remote database
    Remote,
}

impl std::fmt::Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Storage::Memory => write!(f, "memory"),
            Storage::Disk => write!(f, "disk"),
            Storage::Remote => write!(f, "remote"),
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = true)]
    pub local: bool,

    /// Open an interactive session
    #[arg(short, long, default_value_t = false)]
    pub interactive: bool,

    /// Number of times to greet
    #[arg(short, long, default_value_t, value_enum)]
    pub storage: Storage,
}
use structopt::StructOpt;
use crate::cli::CliArguments::Search;
use crate::cli::CliArguments::Scan;

#[derive(Debug)]
#[derive(StructOpt)]
#[structopt(name = "CliArguments", about = "Gestion fichier music")]
pub enum CliArguments {
    #[structopt(name = "scan")]
    Scan {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    #[structopt(name = "search")]
    Search {
        line : Vec<String>,
    }
}

impl CliArguments {
    
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }

    pub fn path(&self) -> Option<&std::path::Path> {
        match self {
            Scan { path } => Some(path.as_path()),
            Search { .. } => None,
        }
    }
}
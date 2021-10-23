use structopt::StructOpt;

/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug)]
#[derive(StructOpt)]
pub struct CliArguments {
    /// Commande à exécuter
    command: String,

    /// Chemin où trouver les fichiers à analyser
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }
}

use structopt::StructOpt;

#[derive(StructOpt,Debug)]
pub enum Action {
    Scan{
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    Search{
        #[structopt(name = "ARGUMENTS")]
        filter: Vec<String>,
    },
    Write2md{
        #[structopt(name = "ARGUMENTS")]
        filter: Vec<String>,
    },
    Write2playlist {
        #[structopt(name = "ARGUMENTS")]
        filter: Vec<String>,
    }
}


/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug)]
#[derive(StructOpt)]
pub struct CliArguments {
    /// Commande à exécuter
    #[structopt(subcommand)]
    pub command: Action,
}

impl CliArguments {
    pub fn default() -> CliArguments {
        CliArguments::from_args()
    }

    pub fn path(&self) -> &std::path::Path {
        match &self.command {
            Action::Scan{path} => path,
            _ => panic!("Erreur"),
        }
    }

    pub fn command(&self) -> String {
        match &self.command {
            Action::Scan{path:_} => "scan".to_owned(),
            Action::Search{filter:_} => "search".to_owned(),
            Action::Write2md {filter:_} => "write2md".to_owned(),
            Action::Write2playlist {filter:_} => "write2playlist".to_owned()
        }
    }

    pub fn request (&self) -> Vec<String> {
        match &self.command {
            Action::Scan{path:_} => None.expect("Ne dispose pas d'élement"),
            Action::Search{filter} => filter.clone(),
            Action::Write2md {filter} => filter.clone(),
            Action::Write2playlist {filter} => filter.clone()
        }
    }
}

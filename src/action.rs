pub enum Action {
    Add { repo: String },
    Update,
    Upgrade { packages: Vec<String> },
    Autoremove,
    Remove { packages: Vec<String> },
    List,
    Search { term: String },
    Clean { packages: Vec<String> },
    Show { package: String },
}

impl Action {
    pub fn parse(args: &[String]) -> Result<Self, String> {
        let cmd = args.get(1)
            .ok_or("no command provided")?
            .as_str();

        match cmd {
            "add" => {
                let repo = args.get(2)
                    .ok_or("add requires <repo>")?
                    .clone();
                Ok(Action::Add { repo })
            }
            "update" => Ok(Action::Update),
            "upgrade" => {
                let packages = args[2..].to_vec();
                Ok(Action::Upgrade { packages })
            }
            "autoremove" => Ok(Action::Autoremove),
            "remove" => {
                let packages = args[2..].to_vec();

                if packages.is_empty() {
                    Err("remove requires a package".into())
                } else {
                    Ok(Action::Remove { packages })
                }
            }
            "list" => Ok(Action::List),
            "search" => {
                let term = args.get(2)
                    .ok_or("search requires <term>")?
                    .clone();
                Ok(Action::Search { term })
            }
            "clean" => {
                let packages = args[2..].to_vec();
                Ok(Action::Clean { packages })
            }
            "show" => {
                let package = args.get(2)
                    .ok_or("show requires <package>")?
                    .clone();
                Ok(Action::Show { package })
            }
            _ => Err(format!("unknown command {}", cmd)),
        }
    }

    pub fn execute(self) {
        match self {
            Action::Add { repo } => add(repo),
            Action::Update => update(),
            Action::Upgrade { packages } => upgrade(packages),
            Action::Autoremove => autoremove(),
            Action::Remove { packages } => remove(packages),
            Action::List => list(),
            Action::Search { term } => search(term),
            Action::Clean { packages } => clean(packages),
            Action::Show { package } => show(package),
        }
    }
}

fn add(repo: String) {
    println!("adding {}", repo);
}

fn update() {
    println!("updating");
}

fn upgrade(packages: Vec<String>) {
    for (_, p) in packages.iter().enumerate() {
        println!("upgrading: {}", p);
    }
}

fn autoremove() {
    println!("autoremoving");
}

fn remove(packages: Vec<String>) {
    for (_, p) in packages.iter().enumerate() {
        println!("removing: {}", p);
    }
}

fn list() {
    println!("listing");
}

fn search(term: String) {
    println!("searching: {}", term);
}

fn clean(packages: Vec<String>) {
    for (_, p) in packages.iter().enumerate() {
        println!("cleaning: {}", p);
    }
}

fn show(package: String) {
    println!("showing {}", package);
}


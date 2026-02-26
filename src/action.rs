use crate::util::{self, yn_prompt};
use git2::Repository;
use std::fs;
use std::path::PathBuf;

pub enum Action {
    Add { url: String },
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
        let cmd = args.get(1).ok_or("no command provided")?.as_str();

        match cmd {
            "add" => {
                let url = args.get(2).ok_or("add requires <repo>")?.clone();
                Ok(Action::Add { url })
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
                let term = args.get(2).ok_or("search requires <term>")?.clone();
                Ok(Action::Search { term })
            }
            "clean" => {
                let packages = args[2..].to_vec();
                Ok(Action::Clean { packages })
            }
            "show" => {
                let package = args.get(2).ok_or("show requires <package>")?.clone();
                Ok(Action::Show { package })
            }
            _ => Err(format!("unknown command {}", cmd)),
        }
    }

    pub fn execute(self) -> Result<(), String> {
        match self {
            Action::Add { url } => add(url.as_str()),
            Action::Update => Ok(update()),
            Action::Upgrade { packages } => Ok(upgrade(packages)),
            Action::Autoremove => Ok(autoremove()),
            Action::Remove { packages } => remove(packages),
            Action::List => Ok(list()),
            Action::Search { term } => Ok(search(term)),
            Action::Clean { packages } => Ok(clean(packages)),
            Action::Show { package } => Ok(show(package)),
        }
    }
}

fn add(url: &str) -> Result<(), String> {
    let base_path = "/var/lib/forge";
    let repo_name = {
        let last_segment = url.rsplit('/').next().unwrap_or(url);
        last_segment.strip_suffix(".git").unwrap_or(last_segment)
    };
    let clone_path = PathBuf::from(base_path).join(repo_name);
    let repo = Repository::clone(url, &clone_path)
        .map_err(|e| format!("failed to clone {}: {}", repo_name, e))?;
    println!(
        "new package initialized at: {}",
        repo.path().to_str().unwrap()
    );
    Ok(())
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

fn remove(packages: Vec<String>) -> Result<(), String> {
    let base_path = "/var/lib/forge";
    println!("checking dependencies...\n");
    let package_paths: Vec<(String, PathBuf)> = packages
        .into_iter()
        .map(|p| {
            let path = PathBuf::from(base_path).join(&p);
            if !path.exists() {
                Err(format!("no installed package: {}", p))
            } else {
                Ok((p, path))
            }
        })
        .collect::<Result<_, _>>()?; // propagates the first error

    println!(
        "Packages to remove ({}): {}\n",
        package_paths.len(),
        package_paths
            .iter()
            .map(|(p, _)| p.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let total_size: u64 = package_paths
        .iter()
        .map(|(_, path)| util::dir_size(path).unwrap_or(0))
        .sum();

    println!(
        "Total remove size: {:.2} MB\n",
        total_size as f64 / (1024.0 * 1024.0)
    );

    if yn_prompt("Proceed with removal?") {
        for (name, path) in package_paths {
            fs::remove_dir_all(&path).map_err(|e| format!("failed to remove {}: {}", name, e))?;
            println!("Removed {}", name);
        }
    }

    Ok(())
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

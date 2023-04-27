use octocrab::Octocrab;

use percent_encoding::{percent_decode_str};
use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, help = "Github access token")]
    token: String,
    #[arg(short, long, help = "Organisation name")]
    org: String,
}

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let args = Args::parse();

    let octocrab = Octocrab::builder().personal_token(args.token).build()?;

    let repos = octocrab.orgs(args.org.clone()).list_repos().send().await.unwrap();

    let path = std::env::current_dir().unwrap().to_string_lossy().to_string();

    println!("Repositories for organization '{}':", args.org);
    for repo in repos {
        let repo_archive_repaired = percent_decode_str(repo.archive_url.as_ref()).decode_utf8_lossy().to_string();
        let repo_url = repo_archive_repaired.replace("{archive_format}", "zipball/").replace("{/ref}", &repo.default_branch.unwrap());

        let repo_path = path.to_owned() + "/" + &repo.name + ".zip";

        println!("Downloading zipball for repo: {} using url: {}", repo.name, repo_url);

        Command::new("wget")
        .arg(repo_url)
        .arg("-q")
        .arg("--show-progress")
        .arg("-O")
        .arg(repo_path)
        .spawn().unwrap().wait().unwrap();
    }

    Ok(())
}

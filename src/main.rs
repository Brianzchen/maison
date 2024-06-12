mod git_undo;

use clap::Parser;
use crate::git_undo::git_undo;

/// Repo maintenance CLI to help you keep your house in order
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The feature of Maison to invoke (ie: loc)
    feature: String,
}

fn main() {
    let args = Args::parse();

    let feature = args.feature.clone();
    let feature = feature.as_str();

    match feature{
        "loc"=> println!("you called loc!"),
        "git-undo"=> git_undo(),
        _=> {
            panic!("You must call a valid feature for maison to run upon, try `maison loc` as an example");
        }
    }
}

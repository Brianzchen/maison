use clap::Parser;

/// Repo maintenance CLI to help you keep your house in order
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The feature of Maison to invoke (ie: loc)
    feature: String,
}

fn main() {
    let args = Args::parse();

    if args.feature == "loc" {
        println!("you called loc!");
    } else {
        panic!("You must call a valid feature for maison to run upon, try `maison loc` as an example")
    }
}

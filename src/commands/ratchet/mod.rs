use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    println!("ratcheting {}", name);
}

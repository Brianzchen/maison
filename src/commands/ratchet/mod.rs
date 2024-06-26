use clap::ArgMatches;

/// Ratchet a value against a name
/// that tags the latest commit with the given value
///
/// The value must be numeric signalling the number of errors acceptable
/// If the value is greater we throw an error
/// If the value is the same we do nothing
/// If the value is lesser
///     We should ...
pub fn run(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    println!("ratcheting {}", name);
}

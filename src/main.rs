use clap::Parser;
use clap_num::number_range;
mod meow;

/// Helper function to validate the command-line numeric argument
fn valid_cat_count(s: &str) -> Result<u16, String> {
    number_range(s, 0, 65535)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Print ASCII cats to your terminal
struct Args {
    /// How many cats to print
    #[arg(short = 'c', long = "count", value_parser = valid_cat_count)]
    count: Option<u16>,

    /// Are you literally this cat?
    #[arg(short = 'l', long = "literally", action)]
    literally: bool,

    /// Type "meow" one or more times to print that many cats (e.g., "meow meow meow" = 3 cats)
    #[arg(trailing_var_arg = true)]
    meow_args: Vec<String>,
}

/// Prints ASCII cats depending on command-line parameters
fn main() {
    let args: Args = Args::parse();

    // Count how many "meow" arguments were provided
    let meow_count = args.meow_args.iter()
        .filter(|arg| arg.to_lowercase() == "meow")
        .count() as u16;

    // Determine the count: use meow count if any, otherwise use -c value or default to 1
    let count = if meow_count > 0 {
        meow_count
    } else {
        args.count.unwrap_or(1)
    };

    meow::print_cats(args.literally, count);
}

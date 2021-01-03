// Main file
mod hyde;

use hyde::{process_cli, HydeCli};

/// Main function
fn main() {
    let args = HydeCli::from_args();
    process_cli(args);
}

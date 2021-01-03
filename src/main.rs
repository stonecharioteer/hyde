// CLI definition
use structopt::StructOpt;

/// Create and Manage Markdown files and their header content.
///
/// Hyde is a tool that helps you make Jekyll-based Markdown files.
/// Hyde can:
///
///     1. Create a new post
///
///     2. Create a new draft
///
///     3. Move draft to post and add the relevant date.
///
///     4. Add metadata to existing post.
///
///     5. Modify title block metadata for a post.
///
/// Hyde also implements some checks and balances:
///
/// Hyde will warn you when:
///
///     1. A post of the same name has been made before
///
///     2. A post of the same permalink has been made before.
///
/// Hyde uses `toml` files for configuration files. It stores its config
/// in `$HOME/.config/hyde/` in Linux and OSX,
/// and in the `%APPDIR%` on Windows.
///
/// Check out the documentation on https://docs.rs/hyde
#[derive(StructOpt)]
struct Cli {
    /// pattern to look for
    pattern: String,
    /// the path to a file to read,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

/// Helper function to print the type of an object.
#[allow(dead_code)]
fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let args = Cli::from_args();
    process_cli(args);
}

/// Process the parameters given from the CLI input.
fn process_cli(args: Cli) {
    type_of(&args);
    println!("path: {:?}, pattern: {}", args.path, args.pattern);
}

// HydeCli definitions.

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
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
/// Hyde has a https://github.com/cheat/cheat compliant cheatsheet embedded into it.
/// Use `hyde cheat` to output the cheat file to STDOUT.
/// You can store it in your personal cheat folder,
///
/// which you can find using `cheat -d | grep ^personal`
///
#[allow(dead_code)]
#[derive(Debug, StructOpt)]
#[structopt(name = "hyde")]
pub struct HydeCli {
    /// Config file name, without the `.toml` extension.
    /// This file will be stored in a location controlled
    /// by the platform: either in `~/.config/hyde/` (Linux and OSX), or,
    /// in `%APPDIR%` in Windows.
    #[structopt(short, long, default_value = "default")]
    name: String,
    /// verbosity, chain multiple to increase the level.
    #[structopt(short, parse(from_occurrences))]
    verbose: u8,
    #[structopt(subcommand)]
    cmd: BaseSubCommands,
    // pattern to look for
    // pattern: String,
    // the path to a file to read,
    // #[structopt(parse(from_os_str))]
    // path: std::path::PathBuf,
}

/// Subcommands for Cli.
#[allow(dead_code)]
#[derive(Debug, StructOpt)]
enum BaseSubCommands {
    /// Command to configure hyde.
    ///
    Config {
        #[structopt(subcommand)]
        cmd: ConfigSubCommands,
    },
    /// Create a new markdown post.
    Create {
        /// Configure the frontmatter.
        /// This will override the default frontmatter you've configured in
        /// the configuration file.
        ///
        /// The format for this is a string.
        #[structopt(short, long, multiple = true)]
        frontmatter: Vec<String>,
        /// post title
        /// This is the actual title of the post.
        /// The filename will be `title.to_lowercase() + ".md"`
        title: String,
    },
    /// Publish a markdown post to the default publish directory.
    Publish {},
    /// Outputs a https://github.com/cheat/cheat compliant cheatsheet
    /// for `hyde`. If you specify the output location, using `-o`,
    /// it will output to that file, or else it outputs to STDOUT.
    ///
    Cheat {
        /// Output to file.
        /// If omitted, hyde will output to STDOUT.
        /// You may use this will `less`, or manually redirect to a file.
        #[structopt(short, long)]
        output: Option<Option<String>>,
    },
}

/// Config Subcommands
#[allow(dead_code)]
#[derive(Debug, StructOpt)]
enum ConfigSubCommands {
    /// command to create a config file.
    /// Use as `hyde config create` or `hyde -n blog config create`
    ///
    Create {},
    /// command to set a config value in the config file.
    /// Use as `hyde config set published.drafts=false`
    Set {
        input: String,
    },
    Get {
        #[structopt(short, long)]
        all: bool,
        key: String,
    },
    Add {
        key: String,
    },
    Remove {
        key: String,
    },
}

/// Process the parameters given from the CLI input.
pub fn process_cli(args: HydeCli) {
    let name: String = args.name;
    match args.cmd {
        BaseSubCommands::Create { frontmatter, title } => {
            create_post(&title, &frontmatter, &name);
        }
        BaseSubCommands::Cheat { output } => match output {
            Some(file_path) => cheat(&file_path),
            _ => {
                cheat(&None);
            }
        },
        BaseSubCommands::Config { cmd } => {
            process_config(&cmd);
        }
        _ => {
            println!("oops, {:?} was unaccounted", &args.cmd);
        }
    };
}

/// Processes configuration commands.
fn process_config(cmd: &ConfigSubCommands) {
    match cmd {
        ConfigSubCommands::Create {} => {
            // create config file.
        }
        ConfigSubCommands::Add { key } => (),
        ConfigSubCommands::Set { input } => (),
        ConfigSubCommands::Remove { key } => (),
        ConfigSubCommands::Get { all, key } => (),
    }
}

// Creates a post using the title and the formatter.
fn create_post(title: &String, frontmatter: &Vec<String>, name: &String) {
    println!(
        "You're creating something: name: {}, frontmatter: {:?}, title: '{}'",
        name, frontmatter, title,
    );
}

// outputs the cheatsheet
fn cheat(output: &Option<String>) {
    let mut out_writer = match output {
        Some(x) => {
            let path = Path::new(x);
            println!("Writing to a file {:?}", &path);
            Box::new(File::create(&path).unwrap()) as Box<dyn Write>
        }
        None => {
            println!("No output path, writing to stdout.");
            Box::new(io::stdout()) as Box<dyn Write>
        }
    };
    match out_writer.write("Woo!".as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("Error! {:?}", e),
    }
}

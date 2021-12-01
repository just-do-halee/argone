use argone::{prelude::*, *};
use std::path::PathBuf;

ARGS! {
    // this area is top of the clap struct meta area.
    #[clap(after_help = "My Application")]
    version = "0.1"
    author = "just-do-halee <just.do.halee@gmail.com>"

    Config {
        file = "examples/Config"
        prefix = "APP"
        panic = ("couldn't find {} file.", "Config")
    }

    Args {
        /// Root directory
        [Config] rootDir: Option<PathBuf> = CURRENT_DIR.clone()

        /// This name will be changed as app-name or APP_NAME
        #[clap(short, long, default_value = "test")]
        appName: String

        /// A level of verbosity, and can be used multiple times
        #[clap(short, long, parse(from_occurrences))]
        verbose: u8
    }

    commands = Sub2
}

COMMANDS! {
    Sub {
        /// third
        Third {
            Args {
                // Test u8
                test: u8
            }
        }
    }
    Sub2 {
        /// first
        #[clap(alias = "foo")]
        First {
            version = "1.0"
            author = "just-do-halee <just.do.halee@gmail.com>"
            Args {
                /// Test String
                test: String
            }
        }
        /// second
        #[clap(aliases = &["baz", "fizz"])]
        Second {
            Args {
                // Test u8
                test: u8
            }
        }
    }
}

fn main() {
    println!("{:#?}", *ARGS);
    println!("{}", CURRENT_EXE.display());
}

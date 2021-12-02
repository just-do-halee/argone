use argone::{prelude::*, *};
use std::path::PathBuf;

ARGS! {
    /// this area is top of the clap struct meta area.
    #[derive(Clone)]
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
        [Config] rootDir: Option<PathBuf> = Some(CURRENT_DIR.clone())

        /// This name will be changed as app-name or APP_NAME
        (short, long, default_value = "test")
        appName: String; // can be separated by ';' or ','

        /// Some List
        (short, long)
        [Config] someList: Vec<String> = vec!["default".to_string()]

        /// A level of verbosity, and can be used multiple times
        (short, long, parse(from_occurrences))
        verbose: u8,
    }

    commands = Sub2
}

COMMANDS! {
    /// this area is top of the command enum meta area.
    #[derive(Copy)]
    Sub {
        /// third
        Third {
            Args {
                // Test u8
                test: u8
            }
        }
    }

    /// same
    Sub2 {
        /// first
        (alias = "foo")
        First {
            version = "1.0"
            author = "just-do-halee <just.do.halee@gmail.com>"
            Args {
                /// Test String
                test: String
            }
        }
        /// second
        (aliases = &["baz", "fizz"])
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

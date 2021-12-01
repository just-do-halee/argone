use argone::{ARGS, COMMANDS};

ARGS! {

    version = "0.1"
    author = "just-do-halee <just.do.halee@gmail.com>"
    about = "this is our application"

    Config {
        // file's extension could be
        // ini | json | json5 | yaml | toml | ron
        file = "examples/Config"
        // arg's env-prefix ex:) APP_...
        prefix = "APP"
    }

    Args {

        /// This is just one required argument.
        /// and three slashes will be an 'about' for clap cli.
        name: String

        /// This argument is connected to Config(file or env).
        /// Template is
        /// [Config] $name: Option<$type> = $default_value
        /// and this works in non-required argument.
        [Config] age: Option<u8> = 12

        // And all arugments are working on clap_derive format.

        /// Exactly same with clap_derive.
        #[clap(short, long)]
        job: Vec<String>

        /// But #[clap(default_value = "..")] doesn't work
        /// to the config arguments. instead, this would work.
        #[clap(short, long, name = "WEIGHT")]
        [Config] weight: Option<u8> = 50

        /// In normal arguments, default_value will work.
        #[clap(short, long, default_value = "1")]
        verbose: u8

    }

    commands = Sub
}

COMMANDS! {
    Sub {

        /// The first subcommand
        First {
            version = "1.0"
            author = "just-do-halee <just.do.halee@gmail.com>"
            Args {
                /// This area is also same.
                test: String
            }
        }

        Second {
            about = "The second subcommand"
            Args {
                test: u8
            }
        }

    }
}

fn main() {
    println!("{:#?}", *ARGS);

    if ARGS.name == "test" {
        println!("name is test.");
    }

    if ARGS.age.unwrap() != 12 {
        println!("age is not matching default 12.");
    }

    for job in &ARGS.job {
        println!("{}", job);
    }

    println!(
        "weight is {}\nverbose is {}",
        ARGS.weight.unwrap(),
        ARGS.verbose
    );

    if let Some(sub) = &ARGS.commands {
        match sub {
            Sub::First { test } => println!("first command: {}", test),
            Sub::Second { test } => println!("second command: {}", test),
        }
    } else {
        println!("none subcommands");
    }
}

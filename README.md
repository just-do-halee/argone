## ***`argone`***

---

Most intuitive global cli maker. *(lazy_static + config-rs + clap)


[![CI][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![Licensed][license-badge]][license-url]
[![Twitter][twitter-badge]][twitter-url]

[ci-badge]: https://github.com/just-do-halee/argone/actions/workflows/rust.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/argone.svg?labelColor=383636
[license-badge]: https://img.shields.io/crates/l/argone?labelColor=383636
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee

[ci-url]: https://github.com/just-do-halee/argone/actions
[twitter-url]: https://twitter.com/do_halee
[crates-url]: https://crates.io/crates/argone
[license-url]: https://github.com/just-do-halee/argone
| [Examples](https://github.com/just-do-halee/argone/tree/main/examples) | [Docs](https://docs.rs/argone) | [Latest Note](https://github.com/just-do-halee/argone/blob/main/CHANGELOG.md) | 

```toml
[dependencies]
argone = "0.5"
```

---

## *Phases*

1. Parsing `clap`.
2. If clap-argument is `None`<br>and the argument has `[Config]` mark<br>then extract data from `Config(file or env)`.
3. When the data is extracted and that was empty,<br>eventually set Default(=) value.

---

# `Example`
##### * ***basics***
```rust
use argone::{prelude::*, *};

ARGONE! {

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
        /// [Config] $name: Option<$type> = Some($default_value) or
        /// [Config] $name: Vec<$type> = vec![$default_values]
        /// and this works in non-required argument.
        [Config] age: Option<u8> = Some(12),
        [Config] job: Vec<String>

        // And all arugments are working on clap_derive format.

        /// Exactly same with clap_derive.
        (short, long)
        parents: Vec<String>

        /// But (default_value = "..") doesn't work to
        /// the config arguments. instead, this would work.
        (short, long, name = "WEIGHT")
        [Config] weight: Option<u8> = Some(50)

        /// In normal arguments, default_value will work.
        (short, long, default_value = "1")
        verbose: u8

    }

    commands = Sub

}

COMMANDS! {
    Sub {

        /// The subcommand
        /// but subcommands do not have config arguments.
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
        println!("job: {}", job);
    }

    for parents in &ARGS.parents {
        println!("parent: {}", parents);
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
```

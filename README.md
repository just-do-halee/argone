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
| [Docs](https://docs.rs/argone) | [Latest Note](https://github.com/just-do-halee/argone/blob/main/CHANGELOG.md) |

```toml
[dependencies]
argone = "0.1"
```

---

## Phases

1. Clap Parse
2. Clap None &\& [Config] marked -> Extract from Config
3. Config None -> set Default(=)

---

# Example

```rust
use argone::{ARGS, COMMANDS};


ARGS! {
    version = "0.1"
    author = "just-do-halee <just.do.halee@gmail.com>"

    Config {
        file = "loxconfig"
        prefix = "LOX"
        panic = ("couldn't find {} file.", "loxconfig")
    }

    Args {
        /// Root directory
        [Config] rootDir: Option<PathBuf> = argone::CURRENT_DIR.clone()

        /// Sets a custom config file
        #[clap(short, long, default_value = "test")]
        name: String

        /// A level of verbosity, and can be used multiple times
        #[clap(short, long, parse(from_occurrences))]
        verbose: u8
    }

    commands = Sub
}

COMMANDS! {
    Sub {

        /// first
        First {
            version = "1.0"
            author = "just-do-halee <just.do.halee@gmail.com>"
            Args {
                /// Test String
                test: String
            }
        }

        /// second
        Second {
            Args {
                // Test u8
                test: u8
            }
        }

    }
}

println!("{:#?}", *ARGS);
```

// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#![allow(unused)]

#[macro_export]
macro_rules! ARGS {
    (@SomeOrNone $yes:expr) => {
        Some($yes)
    };
    (@SomeOrNone) => {
        None
    };
    (@Parse) => {
        Args::parse()
    };
    (@Parse $file:literal, $($prefix:literal)?, $( ($($config_panic:expr),*) )?) => {
        Args::parse_with_config({
            let mut conf = config::Config::default();
            // ini | json | json5 | yaml | toml | ron
            // arg's prefix must be '$prefix'. ex:) LOX_...
            ARGS!(@SetConfig conf, $file, $($prefix)?, $( ($($config_panic),*) )?)
        })
    };
    (@SetConfig $config:expr, $file:literal, ,) => {
        {
            $config.merge(config::File::with_name($file)).ok();
            $config
        }
    };
    (@SetConfig $config:expr, $file:literal, , ($($config_panic:expr),*)) => {
        {
            $config.merge(config::File::with_name($file))
            .unwrap_or_else(|_| panic!($($config_panic),*))
            $config
        }
    };
    (@SetConfig $config:expr, $file:literal, $prefix:literal,) => {
        {
            $config.merge(config::File::with_name($file)).ok();
            $config.merge(config::Environment::with_prefix($prefix)).ok();
            $config
        }
    };
    (@SetConfig $config:expr, $file:literal, $prefix:literal, ($($config_panic:expr),*)) => {
        {
            $config.merge(config::File::with_name($file))
            .unwrap_or_else(|_| panic!($($config_panic),*))
            .merge(config::Environment::with_prefix($prefix))
            .expect("init");
            $config
        }
    };

    (@SetConfig @If $conf:expr, $opts:expr, $ident:ident,,)
    => {
    };
    (@SetConfig @If $conf:expr, $opts:expr, $ident:ident, [Config], $($def_val:expr)?)
    => {
        if $opts.$ident == None {
             $opts.$ident = if let Ok(v) = $conf.get(stringify!($ident)) { Some(v) }
                            else {
                                ARGS!(@SomeOrNone $($def_val)?)
                            };
        }
    };

    (
        @Args {
            #[$meta:meta]
            $(
                $(#[$f_meta:meta])*
                $([$tt:tt])? $name:ident: $ty:ty $(= $default:expr)?,
            )*
        }
    ) => {

        use clap::{AppSettings, Parser, SubCommand};
        use config::{self, Config};

        #[allow(non_snake_case)]
        #[derive(Debug, Parser)]
        #[$meta]
        pub struct Args {
            $(
                $(#[$f_meta])*
                pub $name: $ty,
            )*
        }
        impl Args {
            pub fn parse_with_config(conf: config::Config) -> Self {
                let mut opts = Args::parse();
                $(
                    ARGS!(@SetConfig @If conf, opts, $name, $([$tt])?, $($default)?);
                )*
                opts
            }
        }
    };

    (
        $(version = $version:literal)?
        $(author = $author:literal)?
        $(Config {
            file = $config_file:literal
            $(prefix = $config_prefix:literal)?
            $(panic = ($($config_panic:expr),*))?
        })?
        Args {
            $(
                $(#[$args_meta:meta])*
                $([$tt:tt])? $args_name:ident: $args_ty:ty $(= $args_default:expr)?
            )+
        }
        $(
            $(#[$subcommands_meta:meta])*
            commands = $subcommands:ty
        )?
    ) => {

        ARGS!(
            @Args {
                #[clap($(version = $version,)? $(author = $author)?)]
                $(
                    $(#[$args_meta])*
                    $([$tt])? $args_name: $args_ty $(= $args_default)?,
                )+
                $(
                    $(#[$subcommands_meta])*
                    #[clap(subcommand)]
                    commands: Option<$subcommands>,
                )?
            }
        );

        lazy_static::lazy_static! {
            pub static ref ARGS: Args = ARGS!(@Parse $($config_file, $($config_prefix)?, $( ($($config_panic),*) )?)?);
        }

    };
}

#[macro_export]
macro_rules! COMMANDS {
    (
        $name:ident {
            $(
                $(#[$meta:meta])*
                $command_name:ident {
                $(version = $version:literal)?
                $(author = $author:literal)?
                $(
                    Args {
                        $(
                            $(#[$args_meta:meta])*
                            $args_name:ident: $args_ty:ty
                        )*
                    }
                )?

                $(
                    $(#[$subcommands_meta:meta])*
                    commands = $subcommands:ty
                )?
            })+
        }
    ) => {

        #[allow(non_snake_case)]
        #[derive(Debug, clap::Parser)]
        pub enum $name {
            $(
                $(#[$meta])*
                #[clap($(version = $version,)? $(author = $author)?)]
                $command_name {
                    $($(
                        $(#[$args_meta])*
                        $args_name: $args_ty,
                    )*)?
                    $(
                        $(#[$subcommands_meta])*
                        #[clap(subcommand)]
                        commands: Option<$subcommands>,
                    )?
                },
            )+
        }

    };
}

use std::{env, path::PathBuf};

lazy_static::lazy_static! {
    pub static ref CURRENT_EXE: String = env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    pub static ref CURRENT_DIR: PathBuf = env::current_dir().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    ARGS! {
        version = "0.1"
        author = "just-do-halee <just.do.halee@gmail.com>"

        Config {
            file = "loxconfig"
            prefix = "LOX"
            // panic = ("couldn't find {} file.", "loxconfig")
        }

        Args {
            /// Root directory
            [Config] rootDir: Option<PathBuf> = CURRENT_DIR.clone()

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

    #[test]
    fn it_works() {
        assert_eq!(&format!("{:?}", *ARGS),
                "Args { rootDir: Some(\"/Users/hwakyeom/programs/libs/argone\"), name: \"test\", verbose: 0, commands: None }"
        );
    }
}

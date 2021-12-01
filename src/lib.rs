// Licensed under either of Apache License, Version 2.0 or MIT license at your option.
// Copyright 2021 Hwakyeom Kim(=just-do-halee)

//! # **`argone`**
//!
//! Most intuitive global cli maker. *(lazy_static + config-rs + clap)<br>
//!
//! <a href="https://github.com/just-do-halee/argone/tree/main/examples">Examples</a>

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
        $(#[$main_meta:meta])*
        $(version = $version:literal)?
        $(author = $author:literal)?
        $(about = $about:literal)?
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
                $(#[$main_meta])*
                #[clap($(version = $version,)? $(author = $author,)? $(about = $about)?)]
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
        $($name:ident {
            $(
                $(#[$meta:meta])*
                $command_name:ident {
                $(version = $version:literal)?
                $(author = $author:literal)?
                $(about = $about:literal)?
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
        })+
    ) => {

        $(#[allow(non_snake_case)]
        #[derive(Debug, Clone, PartialEq, Eq, clap::Parser)]
        pub enum $name {
            $(
                $(#[$meta])*
                #[clap($(version = $version,)? $(author = $author,)? $(about = $about)?)]
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
        })+

    };
}

use std::{env, path::PathBuf};

lazy_static::lazy_static! {
    pub static ref CURRENT_EXE: PathBuf = env::current_exe()
        .unwrap()
        .file_name()
        .map(PathBuf::from)
        .unwrap();
    pub static ref CURRENT_DIR: PathBuf = env::current_dir().unwrap();
}

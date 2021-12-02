// Licensed under either of Apache License, Version 2.0 or MIT license at your option.
// Copyright 2021 Hwakyeom Kim(=just-do-halee)

//! # **`argone`**
//!
//! Most intuitive global cli maker. *(lazy_static + config-rs + clap)<br>
//!
//! <a href="https://github.com/just-do-halee/argone/tree/main/examples">Examples</a>

#![allow(unused)]

#[macro_export]
macro_rules! ARGONE {
    (@PassOrB $a:expr, $b:expr) => {
        $a
    };
    (@PassOrB , $b:expr) => {
        $b
    };
    (@Parse) => {
        Args::parse()
    };
    (@Parse $file:literal, $($prefix:literal)?, $( ($($config_panic:expr),*) )?) => {
        Args::parse_with_config({
            let mut conf = config::Config::default();
            // ini | json | json5 | yaml | toml | ron
            // arg's prefix must be '$prefix'. ex:) LOX_...
            ARGONE!(@SetConfig conf, $file, $($prefix)?, $( ($($config_panic),*) )?)
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

    (@SetConfig @If $conf:expr, $opts:expr, $name:ident,)
    => {
    };
    (@SetConfig @If $conf:expr, $opts:expr, [Config] $name:ident, $($default:expr)?)
    => {
        if $opts.$name.is_empty_or_none() {
             $opts.$name = if let Ok(v) = $conf.get(stringify!($name)) {
                                v
                            }
                            else {
                                ARGONE!(@PassOrB $($default)?, $opts.$name)
                            };
        }
    };


    (
        @Args {
            {
                $(#[$meta:meta])*
            }
            $(
                $(#[$f_meta:meta])*
                $([$tt:tt])? $name:ident: $ty:ty$( = $default:expr)?,
            )*
        }
    ) => {
        #[allow(non_snake_case)]
        #[derive(Debug, Parser)]
        $(#[$meta])*
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
                    ARGONE!(@SetConfig @If conf, opts, $([$tt])? $name, $($default)?);
                )*
                opts
            }
        }
    };

    (
        $(#[$main_meta:meta])*
        $(version = $version:literal $(;)?$(,)?)?
        $(author = $author:literal $(;)?$(,)?)?
        $(about = $about:literal $(;)?$(,)?)?
        $(Config {
            file = $config_file:literal $(;)?$(,)?
            $(prefix = $config_prefix:literal $(;)?$(,)?)?
            $(panic = ($($config_panic:expr),*) $(;)?$(,)?)?
        })?
        Args {
            $(
                $(#[$args_meta:meta])*
                $(($($clap_arg:stmt),*))?
                $([$tt:tt])? $args_name:ident: $args_ty:ty $(= $args_default:expr)? $(;)?$(,)?
            )+
        }
        $(
            $(#[$subcommands_meta:meta])*
            commands = $subcommands:ty $(;)?$(,)?
        )?
    ) => {

        ARGONE!(
            @Args {
                {
                    $(#[$main_meta])*
                    #[clap($(version = $version,)? $(author = $author,)? $(about = $about,)?)]
                }
                $(
                    $(#[$args_meta])*
                    $(#[clap($($clap_arg),*)])?
                    $([$tt])? $args_name: $args_ty$(= $args_default)?,
                )+
                $(
                    $(#[$subcommands_meta])*
                    #[clap(subcommand)]
                    commands: Option<$subcommands>,
                )?
            }
        );

        lazy_static! {
            pub static ref ARGS: Args = ARGONE!(@Parse $($config_file, $($config_prefix)?, $( ($($config_panic),*) )?)?);
        }

    };
}

#[macro_export]
macro_rules! COMMANDS {
    (
        $(
            $(#[$main_meta:meta])*
            $name:ident {
            $(
                $(#[$meta:meta])*
                $(($($clap_arg:stmt),*))?
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
        #[derive(Debug, Clone, PartialEq, Eq, Parser)]
        $(#[$main_meta])*
        pub enum $name {
            $(
                $(#[$meta])*
                $(#[clap($($clap_arg),*)])?
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

pub mod prelude {
    pub use clap::{self, Parser};
    pub use config;
    pub use lazy_static::lazy_static;

    use std::any::Any;
    pub trait ArgOne {
        fn is_empty_or_none(&self) -> bool;
    }

    impl<T> ArgOne for Option<T> {
        #[inline]
        fn is_empty_or_none(&self) -> bool {
            self.is_none()
        }
    }

    impl<T> ArgOne for Vec<T> {
        #[inline]
        fn is_empty_or_none(&self) -> bool {
            self.is_empty()
        }
    }
}

use std::{env, path::PathBuf};

prelude::lazy_static! {
    pub static ref CURRENT_EXE: PathBuf = env::current_exe()
        .unwrap()
        .file_name()
        .map(PathBuf::from)
        .unwrap();
    pub static ref CURRENT_DIR: PathBuf = env::current_dir().unwrap();
}

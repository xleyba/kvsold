#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};
use structopt::StructOpt;

use std::path::PathBuf;
use std::process::exit;
use std::env;

use kvs::{KvsError, Result, KvStore};

#[derive(StructOpt, Debug)]
#[structopt(name = "git", about = "the stupid content tracker")]
enum Opt {
    #[structopt(name = "set")]
    Set {
        #[structopt(value_name = "KEY", help = "The key to insert.")]
        key: String,
        #[structopt(value_name = "VALUE", help = "The value to insert.")]
        value: String,
    },
    #[structopt(name = "get", about = "store a value for a key")]
    Get {
        #[structopt(value_name = "KEY", help = "The key to search the value for")]
        key: String,
    },
    #[structopt(name = "rm")]
    Rm {
        #[structopt(value_name = "KEY", help = "The key to search the value for")]
        key: String,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    match Opt::from_args() {
        Opt::Set { key, value } => {
            let mut store = KvStore::new(env::current_dir()?)?;
            store.set(key, value)?;
            Ok(())
        },
        Opt::Rm { key } => {
            let mut store = KvStore::new(env::current_dir()?)?;
            store.set("pepe".to_owned(), "pepote".to_owned())?;
            store.rm(key)?;
            Ok(())
        }
        _ => Ok(()),
    }

    /*
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("store a value for a key")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value for the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the value for a given key")
                .arg(
                    Arg::with_name("KEY")
                        .help("The key to search the value for")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove the given key from the store")
                .arg(
                    Arg::with_name("KEY")
                        .help("The key to remove")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(matches)) => {
            println!("unimplemented");
            exit(1)},
        ("get", Some(matches)) => {
            println!("unimplemented");
            exit(1)},
        ("rm", Some(matches)) => {
            println!("unimplemented");
            exit(1)},
        _ => unreachable!(),
    }

    */
}

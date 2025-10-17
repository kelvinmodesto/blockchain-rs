use std::{fs, mem, process};

use clap::{Command, arg};

use crate::Blockchain;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("Blockchain Demo")
            .version("0.1")
            .author("Monkey D. Luffy <lorem@ipsum.io>")
            .about("Blockchain in rust for learning purposes")
            .subcommand(Command::new("print").about("Print all the blocks"))
            .subcommand(Command::new("clear").about("Clear the blockchain"))
            .subcommand(
                Command::new("add")
                    .about("Add a block in the blockchain")
                    .arg(arg!(<DATA>"'The block data'")),
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("add") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.cmd_add_block(String::from(c))?;
            } else {
                println!("Failure on add block");
                process::exit(1)
            }
        }

        if matches.subcommand_matches("print").is_some() {
            self.cmd_print_chain()?;
        }

        if matches.subcommand_matches("clear").is_some() {
            self.cmd_clear_blockchain()?;
        }

        Ok(())
    }

    pub fn cmd_print_chain(&self) -> Result<()> {
        for b in self.bc.iter() {
            println!("{:?}", b);
        }
        Ok(())
    }

    pub fn cmd_clear_blockchain(&mut self) -> Result<()> {
        let db_default = sled::open("dev/null")?; // void database as a replacement
        let old_db = mem::replace(&mut self.bc.db, db_default);

        drop(old_db); // Free old database
        let db_path = "data/blocks";

        fs::remove_dir_all(db_path)?;

        println!("Blockchain cleared");
        process::exit(0)
    }

    pub fn cmd_add_block(&mut self, data: String) -> Result<()> {
        println!("Adding block with data {:?}", data);
        self.bc.add_block(data)?;
        Ok(())
    }
}

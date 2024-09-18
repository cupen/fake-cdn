use serde::{Serialize};
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "args")]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Command,

    #[structopt(short, long)]
    pub verbose: bool,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Web {
        #[structopt(long)]
        listen: String,
    },
}


pub fn parse_args() -> Args {
    return Args::from_args()
}
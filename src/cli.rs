use serde::{Serialize};
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "args")]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Command,

    // #[structopt(short, long)]
    // pub verbose: bool,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Web {
        #[structopt(long, env="FAKECDN_LISTEN", default_value="127.0.0.1:9527")]
        listen: String,

        #[structopt(long, env="FAKECDN_DIR", default_value=".uploads")]
        dir: String,

        #[structopt(long, env="FAKECDN_TOKEN", default_value="")]
        token: String,
    },
}


pub fn parse_args() -> Args {
    return Args::from_args()
}
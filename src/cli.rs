use structopt::StructOpt;

use std::sync::OnceLock;


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

pub fn get_args() -> &'static Args {
    static ARGS: OnceLock<Args> = OnceLock::new();
    return ARGS.get_or_init(|| parse_args());
}


pub fn get_args_token() -> &'static String {
    let args = get_args();
    match &args.command {
        Command::Web { token, .. } => return token,
    }
}

pub fn parse_args() -> Args {
    return Args::from_args();
}

// pub fn parse_args() -> &'static Mutex<Args> {
//     // return Args::from_args()
//     static ARGS: OnceLock<Mutex<Args>> = OnceLock::new();
//     return ARGS.get_or_init(|| Mutex::new(Args::from_args()))
// }
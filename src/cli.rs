use structopt::StructOpt;
use std::sync::OnceLock;

#[derive(Debug, StructOpt)]
#[structopt(name = "args")]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Command,

    #[structopt(long, env = "FAKECDN_CONFIG", default_value = "conf/conf.toml")]
    pub config: String,

    #[structopt(long, env = "FAKECDN_DIR")]
    pub dir: Option<String>,

    #[structopt(long, env = "FAKECDN_TOKEN")]
    pub token: Option<String>,
}

#[derive(StructOpt, Debug)]
pub enum Command {


    #[structopt(name = "web")]
    Web {
        #[structopt(long, env = "FAKECDN_LISTEN")]
        listen: String,
    },

    #[structopt(name = "install-service")]
    InstallService {
        #[structopt(
            long,
            env = "FAKECDN_SERVICE_PATH",
            default_value = "/etc/systemd/system/fake-cdn.service"
        )]
        path: String,
    },

    #[structopt(name = "init")]
    Init {
        #[structopt(long)]
        dir: Option<String>,
        #[structopt(long)]
        token: Option<String>,
        #[structopt(long)]
        force: bool,
    },
}

pub fn get_args() -> &'static Args {
    static ARGS: OnceLock<Args> = OnceLock::new();
    return ARGS.get_or_init(|| parse_args());
}

pub fn parse_args() -> Args {
    return Args::from_args();
}
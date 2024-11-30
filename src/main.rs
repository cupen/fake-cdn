use colog;
use log::error;
use std::path::Path;

mod cli;
use cli::Command;
mod conf;
mod files;
mod subcmds;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    colog::init();

    let args = cli::get_args();
    let file_conf = conf::load_from_file(Path::new(&args.config));

    let dir = args
        .dir
        .clone()
        .or(file_conf.dir)
        .unwrap_or_else(|| ".uploads".to_string());

    let token = args.token.clone().or(file_conf.token);

    if let Command::Init { dir, token, force } = &args.command {
        return subcmds::init::run(&args.config, dir.as_ref(), token.as_ref(), *force).await;
    }

    let token = match token {
        Some(t) => t,
        None => {
            error!("[main] token is required. Provide it via --token, FAKECDN_TOKEN, config file, or run `init` command.");
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "token is required",
            ));
        }
    };

    return match &args.command {
        Command::Web { listen } => subcmds::web::run(listen, &token, &dir).await,
        Command::InstallService { path } => subcmds::install_service::run(path).await,
        Command::Init { .. } => unreachable!(),
    };
}

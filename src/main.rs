use colog;
use log::{debug, error, info};
use std::io::Read;
use std::net::SocketAddr;
use std::path::Path;
use std::path::PathBuf;

mod cli;
use cli::Command;

mod files;

// use lazy_static::lazy_static;
use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
// use actix_web::http::header::{HeaderMap, HeaderValue};
use actix_files::Files;
use tokio::fs;
// use tokio::fs::File;
// use mime_guess::from_path;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
// use actix_multipart::Multipart;
// use serde::Deserialize;
use std::fmt::Debug;
use serde_json::json;

const UPLOAD_DIR: &str = ".uploads";
// const listen: &str = "127.0.0.1:9527";

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "512MB")]
    file: TempFile,
}

#[post("/{path:.*}")]
async fn upload(
    args: web::Path<(String,)>,
    req: HttpRequest,
    MultipartForm(mut form): MultipartForm<UploadForm>,
) -> impl Responder {
    let fpath = args.into_inner().0;
    if fpath.contains("..") {
        return HttpResponse::BadRequest().body("Bad path");
    }
    debug!("[upload] {:?}", form);
    let token = req.headers().get("Authorization");
    match token {
        Some(t) => {
            info!("[upload] token: ...");
            let token = cli::get_args_token();
            if token.eq(t.to_str().unwrap()) {
                info!("[upload] token: ok");
            } else {
                info!("[upload] token: invalid");
                return HttpResponse::Unauthorized().body("Unauthorized");
            }
        }
        None => {
            info!("[upload] no token");
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
    }
    info!("[upload] {} size: {}", fpath, form.file.size);
    let full_path = PathBuf::from(UPLOAD_DIR).join(fpath.clone());

    let dpath = full_path.parent().unwrap();
    if !dpath.exists() {
        fs::create_dir_all(dpath)
            .await
            .expect("Unable to create dir");
    }

    // if cfg!(windows) {
    let buf: &mut Vec<u8> = &mut Vec::new();
    // let data = form.file.file.as_file();
    let fp = form.file.file.as_file_mut();
    match fp.read_to_end(buf) {
        Ok(s) => {
            info!("[upload]  => saved: {} {} bytes", full_path.display(), s);
            fs::write(full_path.clone(), buf)
                .await
                .expect("Unable to write file");


            let fname = form.file.file_name.unwrap();
            if fname.contains(".") {
                fname.split(".").last().unwrap();
                let mut ext = Path::new(&fname)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap();
                if fname.ends_with(".tar.gz") || fname.ends_with(".tgz") {
                    ext = "tar.gz";
                }
                match ext {
                    "tar.gz" => {
                        info!("[upload] {} is tar.gz",  full_path.display());
                        let full_dir = full_path.parent().unwrap();
                        files::uncompress_tgz(&full_path, full_dir).expect("Unable to uncompress");
                    }
                    "zip" => {
                        info!("[upload] {} is zip", fpath);
                    }
                    "html" => {
                        info!("[upload] {} is html", fpath);
                    }
                    _ => {
                        info!("[upload] {} is {}", fpath, ext);
                    }
                }
            }
        }

        _ => {
            error!("[upload]  => save failed: {}", full_path.display());
            return HttpResponse::BadRequest().body("Bad file");
        }
    }
    return HttpResponse::Ok().body("");
}

#[get("/status")]
async fn status() -> impl Responder {
    info!("[status] ok");
    HttpResponse::Ok().json(json!({ 
        "status": "ok", 
        "version": "0.1.0" 
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    colog::init();

    let args = cli::get_args();
    match &args.command {
        Command::Web { listen, dir, token:_ } => {
            let addr = listen
                .parse::<SocketAddr>()
                .expect("Invalid listen address");
            println!("Listening on: {}", addr);
            // let upload_dir = dir.clone();
            let mut upload_dir = PathBuf::from(dir.clone());
            if dir.eq("") {
                upload_dir = PathBuf::from(UPLOAD_DIR);
            }
            if !upload_dir.exists() {
                error!("--dir {} doesn't exists", upload_dir.display());
            }
            HttpServer::new(move || {
                App::new().service(status).service(upload).service(
                    Files::new("/", &upload_dir)
                        .prefer_utf8(true)
                        .show_files_listing(),
                )
            })
            .bind(addr)?
            .run()
            .await
        }
    }
}

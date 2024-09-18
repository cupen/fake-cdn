use std::io::Read;
use std::net::SocketAddr;
use std::path::PathBuf;

mod cli;
use cli::Command;

// use lazy_static::lazy_static;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
// use actix_web::http::header::{HeaderMap, HeaderValue};
use actix_files::Files;
use tokio::fs;
// use tokio::fs::File;
// use mime_guess::from_path;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
// use actix_multipart::Multipart;
// use serde::Deserialize;
use std::fmt::Debug;

const UPLOAD_DIR: &str = ".uploads";
// const listen: &str = "127.0.0.1:9527";

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "512MB")]
    file: TempFile,
}

// #[post("/")]
// async fn upload(req: HttpRequest) -> impl Responder {
//     println!("http upload: {:?}", req);
//     let path = req.path();
//     if path.contains("..") {    
//         return HttpResponse::BadRequest().body("Bad path")
//     }
//     let full_path = format!("{}/{}", upload_dir, path);
//     // File::create(full_path).await.expect("Unable to create file");
//     fs::write(full_path, req.body()).await.expect("Unable to write file");
//     return HttpResponse::Ok().body("File created");
// }


#[post("/{path:.*}")]
async fn upload(args: web::Path<(String,)>,  MultipartForm(mut form): MultipartForm<UploadForm>) -> impl Responder {
    let fpath = args.into_inner().0;
    if fpath.contains("..") {
        return HttpResponse::BadRequest().body("Bad path");
    }
    let fname = form.file.file_name.unwrap();
    println!("[upload] path:{} fname:{} size:{}", fpath, fname, form.file.size);
    let full_path = PathBuf::from(UPLOAD_DIR).join(fpath);
    let dpath = full_path.parent().unwrap();
    if !dpath.exists() {
        fs::create_dir_all(dpath).await.expect("Unable to create dir");
    }

    if cfg!(windows) {
        let buf :&mut Vec<u8> = &mut Vec::new();
        // let data = form.file.file.as_file();
        let fp = form.file.file.as_file_mut();
        let a = fp.read_to_end(buf).unwrap();
        // let a = fp.read_to_end(buf).unwrap();
        if a == 0 {
            return HttpResponse::BadRequest().body("Bad file");
        }
        fs::write(full_path, buf).await.expect("Unable to write file");
    } else if cfg!(unix) {
        form.file.file.persist(full_path).unwrap();
    }
    return HttpResponse::Ok().body("");
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("[echo] {:?}", req_body);
    HttpResponse::Ok().body(req_body)
}

#[get("/status")]
async fn status() -> impl Responder {
    println!("[status] ok");
    HttpResponse::Ok()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = cli::parse_args();
    match args.command  {
        Command::Web { listen } => {
            let lis = listen.parse::<SocketAddr>().expect("Invalid listen address");
            println!("Listening on: {}", lis);
            HttpServer::new(|| {
                App::new()
                    .service(echo)
                    .service(status)
                    .service(upload)
                    .service(
                        Files::new("/" , UPLOAD_DIR)
                        .prefer_utf8(true)
                        .show_files_listing())
            })
            .bind(lis)?
            .run()
            .await
        },
        _ => {
            panic!("Invalid command");
            // HttpServer::new(|| {
            //     App::new()
            //         .service(echo)
            //         .service(status)
            //         .service(upload)
            //         .service(Files::new("/" , UPLOAD_DIR).prefer_utf8(true))
            // })
            // .run()
            // .await
        }
    }
}

use actix_files::Files;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::{debug, error, info, warn};
use serde_json::json;
use std::fmt::Debug;
use std::io::Read;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs;

const UPLOAD_DIR: &str = ".uploads";

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "1024MB")]
    file: TempFile,
}

#[post("/{path:.*}")]
async fn upload(
    args: web::Path<String>,
    req: HttpRequest,
    MultipartForm(mut form): MultipartForm<UploadForm>,
    token: web::Data<String>,
) -> impl Responder {
    let fpath = args.into_inner();

    if !crate::utils::is_safe_url(&fpath) {
        error!("[upload] bad path: {fpath}");
        return HttpResponse::BadRequest().body("Bad path");
    }

    debug!("[upload] {:?}", form);
    let header_token = req.headers().get("Authorization");
    match header_token {
        Some(t) => {
            if !token.as_ref().eq(t.to_str().unwrap()) {
                warn!("[upload] token: invalid");
                return HttpResponse::Unauthorized().body("Unauthorized");
            }
        }
        None => {
            warn!("[upload] no token");
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

    let buf: &mut Vec<u8> = &mut Vec::new();
    let fp = form.file.file.as_file_mut();
    match fp.read_to_end(buf) {
        Ok(s) => {
            fs::write(full_path.clone(), buf)
                .await
                .expect("[upload] write file failed");

            info!("[upload]   => saved. {} size: {}", full_path.display(), s);
            let fname = form.file.file_name.unwrap();
            if fname.contains(".") {
                let mut ext = crate::utils::file_extension(&fname).unwrap();
                if ext == "tgz" {
                    ext = "tar.gz"
                }
                match ext {
                    "tar.gz" => {
                        debug!("[upload] {} is tar.gz", full_path.display());
                        let full_dir = full_path.parent().unwrap();
                        crate::files::uncompress_tgz(&full_path, full_dir)
                            .expect("Unable to uncompress");
                    }
                    "zip" => {
                        debug!("[upload] {fpath} is zip");
                    }
                    "html" => {
                        debug!("[upload] {fpath} is html");
                    }
                    _ => {
                        debug!("[upload] {fpath} is {ext}");
                    }
                }
            }
        }

        _ => {
            error!("[upload]  => save failed: {}", full_path.display());
            return HttpResponse::BadRequest().body("Bad file");
        }
    }
    return HttpResponse::Ok().body("ok");
}

#[get("/status")]
async fn status() -> impl Responder {
    info!("[status] ok");
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "version": "0.1.0"
    }))
}

pub async fn run(
    listen: &String,
    token: &String,
    upload_dir: &String,
) -> std::io::Result<()> {
    let addr = listen
        .parse::<SocketAddr>()
        .expect("Invalid listen address");
    println!("Listening on: {}", addr);
    let mut _upload_dir = PathBuf::from(upload_dir.clone());
    if upload_dir.is_empty() {
        _upload_dir = PathBuf::from(UPLOAD_DIR);
    }

    if !_upload_dir.exists() {
        error!("--dir {} doesn't exists", _upload_dir.display());

        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("upload_dir: {} not exists", _upload_dir.display()),
        ));
    }
    let app_token = token.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_token.clone()))
            .service(status)
            .service(upload)
            .service(
                Files::new("/", &_upload_dir)
                    .prefer_utf8(true)
                    .show_files_listing(),
            )
    })
    .bind(addr)?
    .run()
    .await
} 
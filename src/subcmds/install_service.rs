use log::{error, info, warn};
use std::env;

pub async fn run(fpath: &String) -> std::io::Result<()> {
    if cfg!(target_os = "linux") {
        info!("os = linux");
        match (env::current_exe(), env::current_dir()) {
            (Ok(exe_path), Ok(work_dir)) => {
                let listen = "127.0.0.1:9527"; // Default listen address for service
                info!("current exe: {}", exe_path.display());
                info!("working directory: {}", work_dir.display());
                info!("web listen: {}", listen);
                let service_content = format!(
                    r#"[Unit]
Description=Fake CDN Service
After=network.target

[Service]
Type=simple
WorkingDirectory={work_dir}
ExecStart={exec_path} web --listen {listen}
Restart=on-failure

[Install]
WantedBy=multi-user.target
"#,
                    exec_path = exe_path.display(),
                    work_dir = work_dir.display(),
                    listen = listen,
                );
                // 写入 systemd service 文件
                let resp = tokio::fs::write(fpath, service_content.as_bytes()).await;
                if !resp.is_ok() {
                    warn!("write failed!");
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "write failed",
                    ));
                }
                println!("Systemd service file written to: {}", fpath);
                println!("You may need to run: sudo systemctl daemon-reload && sudo systemctl enable --now $(basename {})", fpath);
            }
            (Err(e), _) => error!("get current exe path failed: {}", e),
            (_, Err(e)) => error!("get current working directory failed: {}", e),
        }
    } else if cfg!(target_os = "windows") {
        warn!("os = windows");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Windows",
        ));
    } else if cfg!(target_os = "macos") {
        warn!("os = macos");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Windows",
        ));
    } else {
        println!("这是其他操作系统");
    }
    Ok(())
} 
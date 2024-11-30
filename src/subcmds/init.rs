use std::io::{self, Write};
use std::path::Path;
use tokio::fs;

pub async fn run(
    config_path_str: &String,
    dir_arg: Option<&String>,
    token_arg: Option<&String>,
    force: bool,
) -> std::io::Result<()> {
    let config_path = Path::new(config_path_str);

    if !force && config_path.exists() {
        print!("Configuration file '{}' already exists. Overwrite? [y/N]: ", config_path_str);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("Initialization cancelled.");
            return Ok(());
        }
    }

    // Determine dir: use arg or prompt interactively
    let dir = match dir_arg {
        Some(d) => d.to_string(),
        None => {
            print!("Enter upload directory [default: .uploads]: ");
            io::stdout().flush()?;
            let mut dir_input = String::new();
            io::stdin().read_line(&mut dir_input)?;
            dir_input.trim().to_string()
        }
    };
    let dir = if dir.is_empty() { ".uploads" } else { &dir };

    // Determine token: use arg, or generate and prompt
    let token = match token_arg {
        Some(t) => t.to_string(),
        None => {
            let new_token = crate::utils::generate_token(32);
            print!(
                "\nGenerated token: {}\nUse this token? [Y/n], or enter a new one: ",
                new_token
            );
            io::stdout().flush()?;
            let mut token_input = String::new();
            io::stdin().read_line(&mut token_input)?;
            match token_input.trim() {
                "" | "y" | "Y" => new_token,
                "n" | "N" => {
                    print!("Please enter your desired token: ");
                    io::stdout().flush()?;
                    let mut custom_token = String::new();
                    io::stdin().read_line(&mut custom_token)?;
                    custom_token.trim().to_string()
                }
                custom => custom.to_string(),
            }
        }
    };

    // Create conf dir if not exists
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).await?;
        }
    }

    // Write to conf.toml
    let config_content = format!(
        r#"# Fake-CDN Configuration
# Upload directory for files
dir = "{}"

# Authorization token for uploads
token = "{}"
"#,
        dir, token
    );

    fs::write(config_path, config_content).await?;
    println!("\nConfiguration file created successfully at '{}'", config_path_str);

    Ok(())
} 
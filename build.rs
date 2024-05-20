use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let config_dir = out_dir.join("config");
    fs::create_dir_all(&config_dir).unwrap();

    let default_cfg = r#"
number_feature = false 
dollar_sign_feature = false
tabs_feature = false
compress_empty_line_feature = false
    "#;

    let config_file = config_dir.join("ricat_cfg.toml");
    fs::write(&config_file, default_cfg).unwrap();

    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    let target_config_dir = home_dir.join(".config/ricat");
    fs::create_dir_all(&target_config_dir).expect("Failed to create target config directory");

    let target_config_file = target_config_dir.join("ricat_cfg.toml");
    fs::copy(&config_file, &target_config_file).expect("Failed to copy config file");

    println!(
        "cargo:rustc-env=RICAT_CONFIG_DIR={}",
        target_config_dir.display()
    );
    println!(
        "ricat configuration file created at: {}",
        target_config_file.display()
    );
    std::io::stdout().flush().unwrap();
}

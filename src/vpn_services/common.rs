use std::{
    fs::{read_dir, File},
    io::{Read, Write},
};

use anyhow::Result;

pub fn get_openvpn_files(path: &str) -> Result<Vec<String>> {
    Ok(read_dir(path)?
        .flatten()
        .filter(|s| s.path().to_str().is_some())
        .map(|s| s.path().to_str().unwrap().to_owned())
        .filter(|s| s.contains(".ovpn"))
        .collect())
}

// requires auth.txt to be in same dir
pub fn add_auth_txt_to_openvpn_files_in_dir(dir_path: &str) -> Result<()> {
    for file in get_openvpn_files(dir_path)? {
        add_auth_txt_to_openvpn_file(&file, &format!("{}/auth.txt", dir_path))?;
    }
    Ok(())
}

pub fn add_auth_txt_to_openvpn_file(file_path: &str, auth_txt_path: &str) -> Result<()> {
    let mut f = File::options().read(true).open(file_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    content = content.replace(
        "auth-user-pass",
        &format!("auth-user-pass {}", auth_txt_path),
    );
    let mut w_f = File::options().write(true).truncate(true).open(file_path)?;
    w_f.write_all(content.as_bytes())?;
    Ok(())
}

//creates auth.txt in dir_path
pub fn create_auth_txt(dir_path: &str, user: &str, password: &str) -> Result<()> {
    let mut f = File::create(format!("{}/auth.txt", dir_path))?;

    f.write_all(format!("{}\n{}", user, password).as_bytes())?;
    Ok(())
}

use std::{
    fs::{self, create_dir_all, read_dir, remove_file, File},
    io::Write,
};

use anyhow::{anyhow, Result};
use zip::ZipArchive;

pub fn dir_exists_with_content(path: &str) -> bool {
    if let Ok(size_in_bytes) = fs_extra::dir::get_size(path) {
        size_in_bytes > 10 * 1024
    } else {
        false
    }
}

pub async fn download_and_extract_zip(url: &str, out_path: &str) -> Result<()> {
    let client = awc::Client::new();
    let data = client
        .get(url)
        .send()
        .await
        .map_err(|e| anyhow!("{}", e))?
        .body()
        .limit(100 * 1024_usize.pow(2))
        .await
        .map_err(|e| anyhow!("{}", e))?;

    create_dir_all("./tmp")?;
    let zip_file_path = format!("./tmp/{}.zip", uuid::Uuid::new_v4());
    let mut f = File::create(&zip_file_path)?;
    f.write_all(&data)?;

    let f = std::fs::File::open(&zip_file_path)?;
    let mut archive = ZipArchive::new(f)?;
    create_dir_all(out_path)?;

    archive.extract(out_path)?;
    remove_file(zip_file_path)?;

    Ok(())
}

pub fn move_all_files_from_dir_to(dir_path: &str, out_path: &str) -> Result<()> {
    let files = read_dir(dir_path)?;

    for file in files.flatten() {
        if let Some(file_name) = file.file_name().to_str() {
            let new_path = format!("{}/{}", out_path, file_name);
            fs::rename(file.path(), new_path)?;
        }
    }
    Ok(())
}

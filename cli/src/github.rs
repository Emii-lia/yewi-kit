use std::env::temp_dir;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use serde::Deserialize;
use crate::constants::get_repo_config;

struct FileInfo {
  name: String,
  download_url: String
}

#[derive(Deserialize, Debug)]
struct GithubContentItem {
  name: String,
  #[serde(rename = "download_url")]
  download_url: String,
  #[serde(rename = "type")]
  item_type: String,
}

fn execute_curl(url: &str) -> Result<String, Box<dyn Error>> {
  let output = Command::new("curl")
    .arg("-s")
    .arg("-H")
    .arg("Accept: application/vnd.github.v3+json")
    .arg("-H")
    .arg("User-Agent: yewi-cli")
    .arg(url)
    .output()?;

  if !output.status.success() {
    return Err(format!("Failed to fetch URL: {}", url).into());
  }

  let response = String::from_utf8(output.stdout)?;
  Ok(response)
}
fn fetch_dir_contents(
  owner: &str,
  repo: &str,
  branch: &str,
  path: &str,
) -> Result<Vec<FileInfo>, Box<dyn Error>> {
  let api_url = format!(
    "https://api.github.com/repos/{}/{}/contents/{}?ref={}",
    owner, repo, path, branch
  );

  let response = execute_curl(&api_url)?;

  let items: Vec<GithubContentItem> = serde_json::from_str(&response)?;

  let mut files = Vec::new();

  for item in items {
    if item.item_type == "file" {
      files.push(FileInfo {
        name: item.name,
        download_url: item.download_url,
      });
    }
  }

  if files.is_empty() {
    return Err(format!("No files found in the specified directory: {}", path).into());
  }
  Ok(files)
}

fn download_file(url: &str, dest_path: &Path) -> Result<(), Box<dyn Error>> {
  let output = Command::new("curl")
    .arg("-s")
    .arg("-L")
    .arg("-f")
    .arg(url)
    .output()?;

  if !output.status.success() {
    return Err(format!("Failed to download file from URL: {}", url).into());
  }
  if output.stdout.is_empty() {
    return Err(format!("Downloaded file is empty from URL: {}", url).into());
  }
  let mut file = File::create(dest_path)?;
  file.write_all(&output.stdout)?;

  Ok(())
}

fn copy_component_files(
  source: &Path,
  target: &Path,
  component_name: &str,
) -> Result<(), Box<dyn Error>> {
  let dest_component_dir = target.join("src/components").join(component_name);
  if dest_component_dir.exists() {
    fs::remove_dir_all(&dest_component_dir)?;
  }
  fs::create_dir_all(&dest_component_dir)?;

  for entry in fs::read_dir(source)? {
    let entry = entry?;
    let path = entry.path();

    if path.is_file() {
      let file_name = path.file_name().ok_or("Invalid file name")?;
      let dest_file_path = dest_component_dir.join(file_name);
      fs::copy(&path, &dest_file_path)?;
    }
  }

  println!(
    "Component '{}' downloaded to {:?}",
    component_name, dest_component_dir
  );
  Ok(())
}
pub fn download_component(component_name: &str, target_dir: &Path) -> Result<(), Box<dyn Error>> {
  let ( repo_owner, repo_name, repo_branch, _raw_github_url) = get_repo_config();

  let component_path = format!("src/components/{}", component_name);

  println!("Downloading {} component from {}", component_name, repo_name);

  let temp_dir = temp_dir().join(format!("yewi_{}", component_name));

  if temp_dir.exists() {
    fs::remove_dir_all(&temp_dir)?;
  }
  fs::create_dir_all(&temp_dir)?;

  let files = fetch_dir_contents(&repo_owner, &repo_name, &repo_branch, &component_path)?;

  if files.is_empty() {
    return Err(format!("Component '{}' not found", component_name).into());
  }

  for file in files {
    let dest_path = temp_dir.join(&file.name);
    download_file(&file.download_url, &dest_path)?;
    println!("Downloaded file: {}", file.name);
  }
  copy_component_files(&temp_dir, target_dir, component_name)?;

  fs::remove_dir_all(&temp_dir).ok();

  println!("Component '{}' added to your project.", component_name);

  Ok(())
}
use std::env::temp_dir;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::constants::get_repo_config;

const TEMPLATE_REPO: &str = "yew-app-template";

pub(crate) fn create(project_name: &str) -> Result<(), Box<dyn Error>> {
  let project_dir = PathBuf::from(project_name);
  println!(" Creating a new Yew project: {}", project_name);
  println!();

  if project_dir.exists() {
    return Err(format!(
      "Directory '{}' already exists.",
      project_name
    ).into());
  }

  fs::create_dir_all(&project_dir)?;
  clone_template(&project_dir)?;

  println!();
  println!(" Success! Created '{}' project.", project_name);
  println!();
  println!("Next steps:");
  println!("1. cd {}", project_name);
  println!("2. yarn && yarn build");
  println!("3. cargo add yew web-sys wasm-logger yew-router gloo");
  println!("4. trunk serve");
  println!();
  println!("Then add components with: yewi add <component-name>");

  Ok(())
}

fn clone_template(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  println!(" Downloading template...");

  let git_available = is_git_available();
  println!("  Git available: {}", git_available);

  if git_available {
    println!("  Using git clone...");
    match clone_with_git(project_dir) {
      Ok(_) => {
        println!("  Git clone successful");
      }
      Err(e) => {
        println!("  Git clone failed: {}", e);
        println!("  Falling back to API download...");
        clone_with_api(project_dir)?;
      }
    }
  } else {
    println!("  Git not found, using API download method...");
    clone_with_api(project_dir)?;
  }

  let git_dir = project_dir.join(".git");
  if git_dir.exists() {
    fs::remove_dir_all(git_dir).ok();
  }
  println!("  Template ready");
  Ok(())
}

fn is_git_available() -> bool {
  let paths = vec![
    "git",
    "/usr/bin/git",
    "/usr/local/bin/git",
    "C:\\Program Files\\Git\\cmd\\git.exe",
  ];

  for git_path in paths {
    if let Ok(output) = Command::new(git_path)
      .arg("--version")
      .output()
    {
      if output.status.success() {
        return true;
      }
    }
  }

  false
}

fn clone_with_git(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  let (repo_owner, _repo_name, _repo_branch, _raw_github_url) = get_repo_config();
  let repo_url = format!(
    "https://github.com/{}/{}.git",
    repo_owner, TEMPLATE_REPO
  );
  let parent_dir = project_dir
    .parent()
    .ok_or("Failed to get parent directory")?;
  let project_name = project_dir
    .file_name()
    .ok_or("Failed to get project directory name")?;

  let git_paths = vec!["git", "/usr/bin/git", "/usr/local/bin/git"];

  for git_cmd in git_paths {
    let result = Command::new(git_cmd)
      .arg("clone")
      .arg("--depth")
      .arg("1")
      .arg(&repo_url)
      .arg(project_name)
      .current_dir(parent_dir)
      .output();

    match result {
      Ok(output) => {
        if output.status.success() {
          return Ok(());
        } else {
          let stderr = String::from_utf8_lossy(&output.stderr);
          return Err(format!(
            "Failed to clone: {}\n{}",
            &repo_url, stderr
          ).into());
        }
      }
      Err(_) => {
        continue;
      }
    }
  }

  Err("Git not found in any standard location. Please install git: https://git-scm.com/downloads".into())
}

fn clone_with_api(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  if !is_curl_available() {
    return Err(
      "curl is not installed. Please install curl or use git to clone the template.\n\
       Install: https://curl.se/download.html".into()
    );
  }

  let (repo_owner, _repo_name, repo_branch, _raw_github_url) = get_repo_config();
  let temp_zip = temp_dir().join("yew_app_template.zip");

  let zip_url = format!(
    "https://github.com/{}/{}/archive/refs/heads/{}.zip",
    repo_owner, TEMPLATE_REPO, repo_branch
  );

  println!("  From: {}", zip_url);
  download_file(&zip_url, &temp_zip)?;

  extract_zip(&temp_zip, project_dir)?;

  fs::remove_file(&temp_zip).ok();

  Ok(())
}

fn is_curl_available() -> bool {
  Command::new("curl")
    .arg("--version")
    .output()
    .map(|output| output.status.success())
    .unwrap_or(false)
}

fn download_file(url: &str, dest_path: &Path) -> Result<(), Box<dyn Error>> {
  let output = Command::new("curl")
    .arg("-s")
    .arg("-L")
    .arg("-f")
    .arg(url)
    .output()?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(format!(
      "Failed to download from: {}\n{}",
      url, stderr
    ).into());
  }

  if output.stdout.is_empty() {
    return Err("Downloaded file is empty. Check if the URL is correct.".into());
  }

  let mut file = fs::File::create(dest_path)?;
  std::io::Write::write_all(&mut file, &output.stdout)?;

  Ok(())
}

fn extract_zip(
  zip_path: &Path,
  extract_to: &Path,
) -> Result<(), Box<dyn Error>> {
  let file = fs::File::open(zip_path)?;
  let mut archive = zip::ZipArchive::new(file)?;

  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let file_path = file
      .enclosed_name()
      .ok_or("Invalid file name in zip")?;

    let relative_path = file_path
      .components()
      .skip(1)
      .collect::<PathBuf>();

    let outpath = extract_to.join(&relative_path);

    if file.is_dir() {
      fs::create_dir_all(&outpath)?;
    } else {
      if let Some(p) = outpath.parent() {
        if !p.exists() {
          fs::create_dir_all(p)?;
        }
      }
      let mut outfile = fs::File::create(&outpath)?;
      std::io::copy(&mut file, &mut outfile)?;
    }
  }

  Ok(())
}
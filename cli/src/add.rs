use std::error::Error;
use std::path::PathBuf;
use crate::github::download_component;
use crate::utils::{component_exists, update_components_mod, update_components_scss};

pub(crate) fn add(component_name: &str) -> Result<(), Box<dyn Error>> {
  let project_dir = PathBuf::from(".");

  if !project_dir.join("src").exists() {
    return Err("This command must be run inside a Yewi project directory.".into());
  }

  if component_exists(&project_dir, component_name) {
    return Err(format!(
      "'{}' component already exists in the project.",
      component_name
    ).into());
  }
  println!("Adding '{}' component...", component_name);
  println!();

  download_component(component_name, &project_dir)?;

  println!();

  update_components_mod(&project_dir, component_name)?;
  update_components_scss(&project_dir, component_name)?;

  println!();
  println!("'{}' component added to your project.", component_name);

  Ok(())
}
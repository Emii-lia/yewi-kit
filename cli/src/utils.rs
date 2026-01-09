use std::fs;
use std::path::Path;

pub fn update_components_mod(project_dir: &Path, component_name: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mod_path = project_dir.join("src/components/mod.rs");

  fs::create_dir_all(project_dir.join("src/components"))?;

  let mut content = if mod_path.exists() {
    fs::read_to_string(&mod_path)?
  } else {
    String::new()
  };

  let mod_declaration = format!("mod {};", component_name);
  let pub_use = format!("pub use {}::*;", component_name);

  if !content.contains(&mod_declaration) {
    content.push_str(&format!("{}\n", mod_declaration));
  }

  if !content.contains(&pub_use) {
    content.push_str(&format!("{}\n", pub_use));
  }

  fs::write(&mod_path, content)?;
  println!("Updated src/components/mod.rs");

  Ok(())
}

pub fn update_components_scss(project_dir: &Path, component_name: &str) -> Result<(), Box<dyn std::error::Error>> {
  let styles_dir = project_dir.join("src/styles");
  fs::create_dir_all(&styles_dir)?;

  let scss_path = styles_dir.join("components.scss");

  let mut content = if scss_path.exists() {
    fs::read_to_string(&scss_path)?
  } else {
    String::new()
  };

  let import_statement = format!("@use \"../components/{}/{}\";", component_name, component_name);

  if !content.contains(&import_statement) {
    content.push_str(&format!("{}\n", import_statement));
  }

  fs::write(&scss_path, content)?;
  println!("Updated src/styles/components.scss");

  Ok(())
}

pub fn component_exists(project_dir: &Path, component_name: &str) -> bool {
  project_dir
    .join("src/components")
    .join(component_name)
    .exists()
}
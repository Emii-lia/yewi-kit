#[derive(Clone, PartialEq)]
pub enum PackageManager {
  Npm,
  Yarn,
  Pnpm,
  Bun
}

impl PackageManager {
  pub fn to_string(&self) -> String {
    match self {
      PackageManager::Npm => "npm".to_string(),
      PackageManager::Yarn => "yarn".to_string(),
      PackageManager::Pnpm => "pnpm".to_string(),
      PackageManager::Bun => "bun".to_string(),
    }
  }

  pub fn get_command(&self) -> String {
    match self {
      PackageManager::Npm => "npm i && npm run build".to_string(),
      PackageManager::Yarn => "yarn && yarn build".to_string(),
      PackageManager::Pnpm => "pnpm install && pnpm run build".to_string(),
      PackageManager::Bun => "bun install && bun run build".to_string(),
    }
  }
  pub fn from_str(package_manager: &str) -> Self {
    match package_manager.to_lowercase().as_str() {
      "npm" => PackageManager::Npm,
      "yarn" => PackageManager::Yarn,
      "pnpm" => PackageManager::Pnpm,
      "bun" => PackageManager::Bun,
      _ => PackageManager::Npm,
    }
  }
  pub fn iter() -> impl Iterator<Item = PackageManager> {
    vec![
      PackageManager::Npm,
      PackageManager::Yarn,
      PackageManager::Pnpm,
      PackageManager::Bun
    ].into_iter()
  }
  pub fn get_packages() -> Vec<String> {
    Self::iter().map(|package_manager| package_manager.to_string()).collect()
  }
}
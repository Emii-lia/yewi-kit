pub mod yewi_cli;
pub mod yewi_seo;

use yew::{Html};
use yew_icons::IconData;

#[derive(Clone, PartialEq)]
pub struct EcosystemHeaderProps {
  pub title: String,
  pub description: String,
  pub tags: Vec<&'static str>,
  pub github: String,
  pub crates: String
}

#[derive(Clone, PartialEq)]
pub struct EcosystemExampleProps {
  pub subtitle: String,
  pub codes: Vec<(Option<String>, String)>,
}

#[derive(Clone, PartialEq)]
pub struct EcosystemFeatureItem {
  pub icon: IconData,
  pub title: String,
  pub description: String,
}

#[derive(Clone, PartialEq)]
pub struct EcosystemFeaturesProps {
  pub features: Vec<EcosystemFeatureItem>
}

#[derive(Clone, PartialEq)]
pub struct EcosystemInstallationProps {
  pub subtitle: String,
  pub codes: Vec<String>
}

#[derive(Clone, PartialEq)]
pub struct EcosystemOverviewProps {
  pub problem: Html,
  pub solution: Html,
  pub benefits: Vec<String>
}

#[derive(Clone, PartialEq)]
pub struct EcosystemPurposeProps {
  pub why: String,
  pub ecosystem: String,
}

pub trait Ecosystem {
  fn name() -> String;
  fn header() -> EcosystemHeaderProps;
  fn overview() -> EcosystemOverviewProps;
  fn example() -> EcosystemExampleProps;
  fn installation() -> EcosystemInstallationProps;
  fn purpose() -> EcosystemPurposeProps;
  fn features() -> EcosystemFeaturesProps;
}
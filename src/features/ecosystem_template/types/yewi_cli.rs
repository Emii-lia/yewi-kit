use yew::html;
use yew_icons::IconData;
use crate::features::ecosystem_template::types::{Ecosystem, EcosystemExampleProps, EcosystemFeatureItem, EcosystemFeaturesProps, EcosystemHeaderProps, EcosystemInstallationProps, EcosystemOverviewProps, EcosystemPurposeProps};

pub struct EcosystemYewiCli;

impl Ecosystem for EcosystemYewiCli {
  fn name() -> String {
    "Yewi CLI".to_owned()
  }

  fn header() -> EcosystemHeaderProps {
    EcosystemHeaderProps {
      title: EcosystemYewiCli::name(),
      description: "The official CLI for Yewi Kit. Create projects, install components and manage your Yew applications from the terminal.".to_string(),
      tags: vec!["Rust", "Clap"],
      github: "https://github.com/Emii-lia/yewi-cli".to_string(),
      crates: "https://crates.io/crates/yewi-cli".to_string(),
    }
  }

  fn overview() -> EcosystemOverviewProps {
    EcosystemOverviewProps {
      problem: html! {
        <p>{"Bootstrapping a Yew project means wiring Trunk, Tailwind, a package manager and a component structure by hand. Every new app repeats the same setup, and reusable UI ends up locked inside opaque crates you can't easily read or modify."}</p>
      },
      solution: html! {
        <p>{"Yewi CLI scaffolds a production-ready Yew application in one command and installs components directly into your source tree, which is inspired by shadcn/ui. There is no runtime, no hidden dependency graph ; just files you own, wired to Tailwind and Trunk out of the box."}</p>
      },
      benefits: vec![
        "Zero-config project setup with Tailwind and Trunk pre-wired.".to_string(),
        "Components are copied into your repo ; fully editable, no vendor lock-in.".to_string(),
        "Interactive prompts and curated themes for a fast first run.".to_string(),
        "Rust-native workflow: install, scaffold, build.".to_string(),
      ]
    }
  }

  fn example() -> EcosystemExampleProps {
    EcosystemExampleProps {
      subtitle: "Create and manage yewi projects with yewi CLI".to_string(),
      codes: vec![
        (Some("Create a new project".to_string()), "yewi new my-app".to_string()),
        (Some("Setup tailwind and run trunk dev".to_string()), r#"cd my-app
yewi install
trunk serve"#.to_string()),
        (Some("Add a component".to_string()), "yewi add button".to_string()),
        (Some("Change theme".to_string()), "yewi set --theme 'blue'".to_string()),
        (Some("Change node package manager".to_string()), "yewi set --package bun".to_string()),
      ]
    }
  }
  fn installation() -> EcosystemInstallationProps {
    EcosystemInstallationProps {
      subtitle: "Install Yewi CLI via Cargo to your system".to_string(),
      codes: vec![
        "cargo install yewi-cli".to_string(),
      ]
    }
  }

  fn purpose() -> EcosystemPurposeProps {
    EcosystemPurposeProps {
      ecosystem: EcosystemYewiCli::name(),
      why: "Most web toolchains hide build setups behind complex runtimes and rigid framework layers. Yewi CLI gives you a seamless, end-to-end workflow built purely for Rust: spin up a fully configured Yew and Tailwind project with a single command, then drop in clean, copy-paste UI components as you build. It exists to remove the ceremony so you can focus on building WebAssembly apps at native speed.".to_string()
    }
  }

  fn features() -> EcosystemFeaturesProps {
    EcosystemFeaturesProps {
      features: vec![
        EcosystemFeatureItem {
          title: "Create projects".to_string(),
          description: "Scaffold a production-ready Yew app in one command.".to_string(),
          icon: IconData::LUCIDE_FOLDER_PLUS
        },
        EcosystemFeatureItem {
          title: "Add components".to_string(),
          description: "Install individual components directly into your source.".to_string(),
          icon: IconData::LUCIDE_PACKAGE
        },
        EcosystemFeatureItem {
          title: "Theme selection".to_string(),
          description: "Pick from curated themes or provide your own hex palette.".to_string(),
          icon: IconData::LUCIDE_PALETTE
        },
        EcosystemFeatureItem {
          title: "Interactive prompts".to_string(),
          description: "Guided setup with sensible defaults at every step".to_string(),
          icon: IconData::LUCIDE_MESSAGE_SQUARE
        },
        EcosystemFeatureItem {
          title: "Zero configuration".to_string(),
          description: "Tailwind, Trunk and package manager wired out of the box.".to_string(),
          icon: IconData::LUCIDE_SETTINGS
        },
        EcosystemFeatureItem {
          title: "Component ownership".to_string(),
          description: "Generated code lives in your repo, fully editable.".to_string(),
          icon: IconData::LUCIDE_FILE_CODE
        },
        EcosystemFeatureItem {
          title: "Automatic imports".to_string(),
          description: "Modules and re-exports updated when you add components.".to_string(),
          icon: IconData::LUCIDE_IMPORT
        },
        EcosystemFeatureItem {
          title: "Modern architecture".to_string(),
          description: "Component-driven layout ready to scale.".to_string(),
          icon: IconData::LUCIDE_LAYERS
        }
      ]
    }
  }
}
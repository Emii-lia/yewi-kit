use yew::html;
use yew_icons::IconData;
use crate::features::ecosystem_template::types::{
  Ecosystem,
  EcosystemExampleProps,
  EcosystemFeatureItem,
  EcosystemFeaturesProps,
  EcosystemHeaderProps,
  EcosystemInstallationProps,
  EcosystemOverviewProps,
  EcosystemPurposeProps
};

pub struct EcosystemYewiSeo;

impl Ecosystem for EcosystemYewiSeo {
  fn name() -> String {
    "Yewi SEO".to_string()
  }

  fn header() -> EcosystemHeaderProps {
    EcosystemHeaderProps {
      title: EcosystemYewiSeo::name(),
      description: "First-class SEO for Yew. Declarative metadata, procedural macros and typed APIs that bring modern document head management to Rust WebAssembly applications.".to_string(),
      tags: vec!["Rust", "Yew", "WebAssembly", "Procedural macros"],
      github: "https://github.com/Emii-lia/yewi-seo".to_string(),
      crates: "https://crates.io/crates/yewi-seo".to_string(),
    }
  }

  fn overview() -> EcosystemOverviewProps {
    EcosystemOverviewProps {
      problem: html! {
        <p>{"SEO in Rust WebAssembly apps usually means reaching into web_sys and mutating the document head by hand, verbose, untyped, and easy to get wrong. Metadata drifts away from the components that own it and errors only surface at runtime."}</p>
      },
      solution: html! {
        <p>{"Yewi SEO exposes a typed, declarative API for document metadata. A single"} <span class="ecosystem-overview-highlight">{"#[seo(...)]"}</span> {"attribute macro attaches titles, descriptions, Open Graph, Twitter Cards, links and icons to a component — verified by the Rust compiler."}</p>
      },
      benefits: vec![
        "Compile-time validation of metadata fields.".to_string(),
        "Metadata lives next to the component that owns it.".to_string(),
        "Full Open Graph, Twitter Cards and link tag support.".to_string(),
        "Runtime helpers for dynamic routes and data-driven pages.".to_string(),
      ]
    }
  }

  fn example() -> EcosystemExampleProps {
    EcosystemExampleProps {
      subtitle: "Replace imperative document mutation with a single declaration.".to_string(),
      codes: vec![
        (
          Some("Before".to_string()),
          r#"fn set_meta() {
  let document = window().unwrap().document().unwrap();
  document.set_title("My Page");
  // ...many lines of manual head mutation
}
          "#.to_string(),
        ),
        (
          Some("After".to_string()),
          r#"#[seo(
  meta(
    title = "My Page",
    description = "This is my page description.",
    keywords = "yewi-kit, yewi-seo, yew, rust"
  ),
  open_graph(
    title = "My Page",
    description = "This is my page description.",
    image = "https://example.com/image.png"
  ),
  icon(
    ( rel = "icon", href = "/favicon.ico" ),
    ( rel = "apple-touch-icon", href = "/apple-touch-icon.png")
  )
)]
#[component(MyPage)]
fn my_page() -> Html {
  html! {
    <main>{"Hello"}</main>
  }
}
        "#.to_string()
        )
      ]
    }
  }

  fn installation() -> EcosystemInstallationProps {
    EcosystemInstallationProps {
      subtitle: "Add Yewi SEO to your Yew project.".to_string(),
      codes: vec![
        "cargo add yewi-seo".to_string()
      ]
    }
  }

  fn purpose() -> EcosystemPurposeProps {
    EcosystemPurposeProps {
      ecosystem: EcosystemYewiSeo::name(),
      why: "Yew applications shouldn't have to sacrifice discoverability because they're compiled to WebAssembly. Yewi SEO exists to make metadata feel like part of the component model instead of an afterthought.".to_string()
    }
  }

  fn features() -> EcosystemFeaturesProps {
    EcosystemFeaturesProps {
      features: vec![
        EcosystemFeatureItem {
          title: "#[seo(...)]".to_string(),
          description: "Attach metadata to a component with a single attribute macro.".to_string(),
          icon: IconData::LUCIDE_CODE_2
        },
        EcosystemFeatureItem {
          title: "Typed metadata".to_string(),
          description: "Every field is validated by the Rust compiler.".to_string(),
          icon: IconData::LUCIDE_TYPE
        },
        EcosystemFeatureItem {
          title: "Open Graph".to_string(),
          description: "Open graph support for rich social previews.".to_string(),
          icon: IconData::LUCIDE_SHARE_2
        },
        EcosystemFeatureItem {
          title: "Twitter".to_string(),
          description: "Summary and summary_large_image card out of the box".to_string(),
          icon: IconData::LUCIDE_TWITTER
        },
        EcosystemFeatureItem {
          title: "Link tags".to_string(),
          description: "Canonical, manifest and author".to_string(),
          icon: IconData::LUCIDE_LINK_2
        },
        EcosystemFeatureItem {
          title: "Icons".to_string(),
          description: "Favicons, apple-touch-icon, ...".to_string(),
          icon: IconData::LUCIDE_IMAGE
        },
        EcosystemFeatureItem {
          title: "Compile-time API".to_string(),
          description: "Invalid metadata fails to build, not at runtime".to_string(),
          icon: IconData::LUCIDE_WRENCH
        },
        EcosystemFeatureItem {
          title: "Runtime helpers".to_string(),
          description: "Programmatic updates for dynamic routes and data.".to_string(),
          icon: IconData::LUCIDE_CPU
        }
      ]
    }
  }
}
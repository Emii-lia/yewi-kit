use std::fs;
use std::path::Path;

const BASE_URL: &str = "https://yewi.pages.dev";

fn main() {
  let routes = &[
    "/",
    "/docs",
    "/docs/installation",
    "/docs/quick-start",
    "/docs/avatar",
    "/docs/avatar-group",
    "/docs/badge",
    "/docs/button",
    "/docs/card",
    "/docs/carousel",
    "/docs/checkbox",
    "/docs/collapse",
    "/docs/divider",
    "/docs/dropdown",
    "/docs/file-input",
    "/docs/input",
    "/docs/modal",
    "/docs/password-input",
    "/docs/progress",
    "/docs/radio",
    "/docs/select",
    "/docs/table",
    "/docs/tabs",
    "/docs/textarea",
    "/docs/toast",
  ];
  let mut xml = String::from(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    "#
  );

  for path in routes {
    xml.push_str(&format!(r#"<url>
        <loc>{}{}</loc>
        <changefreq>monthly</changefreq>
        <priority>{}</priority>
      </url>
      "#, BASE_URL, path, if *path == "/" { "1" } else { "0.8" }));
  }
  xml.push_str("</urlset>");
  fs::create_dir_all("dist").unwrap();
  fs::create_dir_all("public").unwrap();
  fs::write(Path::new("dist/sitemap.xml"), &xml).unwrap();
  fs::write(Path::new("public/sitemap.xml"), &xml).unwrap();

  println!("✓ sitemap.xml generated");
}
use yew::{html, Html};

#[derive(Clone)]
pub struct Step {
  pub number: u8,
  pub title: String,
  pub description: String,
  pub command: String,
  pub hint: Option<String>,
  pub children: Option<Html>
}

pub fn get_steps() -> Vec<Step> {
  vec![
    Step {
      number: 0,
      title: "Install Wasm target and Trunk".to_string(),
      description: "You'll first need to install wasm target and trunk in order to compile and run Yew application".to_string(),
      command: r#"rustup target add wasm32-unknown-unknown
cargo install --locked trunk"#.to_string(),
      hint: Some("".to_string()),
      children: Some(html! {
        <div class="custom-install">
          {"If you don't have Rust and Cargo installed, please visit "}
          <a
            href="https://www.rust-lang.org/tools/install"
            target="_blank"
            class="install-link"
            rel="noopener noreferrer"
          >{"Rust installation guide"}</a>
          {" to set up your Rust environment."}
        </div>
      })
    },
    Step {
      number: 1,
      title: "Install Yewi CLI".to_string(),
      description: "The easiest way to get started with Yewi-kit is using the Yewi CLI tool.".to_string(),
      command: "cargo install yewi-cli".to_string(),
      hint: Some("This command installs the Yewi CLI globally on your system, allowing you to create and manage Yewi projects easily.".to_string()),
      children: None
    },
    Step {
      number: 2,
      title: "Create new project".to_string(),
      description: "Use the CLI to initialize a new Yew project with Yewi-kit pre-configured.".to_string(),
      command: "yewi new my-yewi-app".to_string(),
      hint: Some("This scaffolds a new project with all dependencies ready to use.".to_string()),
      children: Some(html! {
        <div class="custom-install">
          {"You can directly specify the theme and i18n usages with the arguments: "}
          <code class="code-highlight">{"--theme <theme-name>|<hex>"}</code>
          {" and "}
          <code class="code-highlight">{"--i18n"}</code>
          {" respectively."}
        </div>
      })
    },
    Step {
      number: 3,
      title: "Install node modules and start development server".to_string(),
      description: "Navigate into the project directory and run the following commands to install dependencies and start the development server.".to_string(),
      command: r#"cd my-yewi-app
yarn install && yarn build
trunk serve"#.to_string(),
      hint: Some(r#"Ensure you have yarn and trunk installed on your system.
                    Your application will be available at http://localhost:8081."#.to_string()),
      children: None
    },
    Step {
      number: 4,
      title: "Add components to your project".to_string(),
      description: "Use the CLI to add components to your project.".to_string(),
      command: "yewi add <component-names>".to_string(),
      hint: Some("Replace <component-names> with the components you want to add, separated by spaces.".to_string()),
      children: None
    },
    Step {
      number: 5,
      title: "Start coding!".to_string(),
      description: "Start building your application using the components you added.".to_string(),
      command: "".to_string(),
      hint: None,
      children: None
    }
  ]
}
#[derive(Clone)]
pub struct Step {
  pub number: u8,
  pub title: String,
  pub description: String,
  pub command: String,
  pub hint: Option<String>
}

pub fn get_steps() -> Vec<Step> {
  vec![
    Step {
      number: 1,
      title: "Install Yewi CLI".to_string(),
      description: "The easiest way to get started with Yewi-kit is using the Yewi CLI tool.".to_string(),
      command: "cargo install yewi-cli".to_string(),
      hint: Some("Ensure you have Rust and Cargo installed on your system.".to_string())
    },
    Step {
      number: 2,
      title: "Create new project".to_string(),
      description: "Use the CLI to initialize a new Yew project with Yewi-kit pre-configured.".to_string(),
      command: "yewi new my-yewi-app".to_string(),
      hint: Some("This scaffolds a new project with all dependencies ready to use.".to_string())
    },
    Step {
      number: 3,
      title: "Install node modules and start development server".to_string(),
      description: "Navigate into the project directory and run the following commands to install dependencies and start the development server.".to_string(),
      command: r#"cd my-yewi-app
yarn install && yarn build
trunk serve"#.to_string(),
      hint: Some(r#"Ensure you have yarn and trunk installed on your system.
                    Your application will be available at http://localhost:8081."#.to_string())
    },
    Step {
      number: 4,
      title: "Add components to your project".to_string(),
      description: "Use the CLI to add components to your project.".to_string(),
      command: "yewi add <component-names>".to_string(),
      hint: Some("Replace <component-names> with the components you want to add, separated by spaces.".to_string())
    },
    Step {
      number: 5,
      title: "Start coding!".to_string(),
      description: "Start building your application using the components you added.".to_string(),
      command: "".to_string(),
      hint: None
    }
  ]
}
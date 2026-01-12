use yew::{function_component, html, Html};

#[function_component(InstallationSection)]
pub(crate) fn installation_section() -> Html {
  html! {
    <div class="InstallationSection">
      <h1 class="doc-title">{"Yewi-kit"}</h1>
      <p class="doc-section-text">
        {r#"
          Yewi-kit is a component library for building web applications using the Yew framework in Rust.
          It provides a set of pre-designed and customizable UI components that follow modern design principles,
          and a project structure mirroring popular frontend frameworks like Next.js.
        "#}
      </p>
      <div class="doc-section">
        <h1 class="doc-section-title">{"Installation"}</h1>
        <div class="doc-subsection">
          <h2 class="doc-subsection-title">{"Yewi CLI"}</h2>
          <p class="doc-section-text">
            {r#"
              The easiest way to get started with Yewi-kit is by using the Yewi CLI tool, which sets up a new Yew project
              with Yewi-kit pre-configured. To install the
            "#}
            <a
              href="https://crates.io/crates/yewi-cli"
              target="_blank"
              rel="noreferrer"
            >
              {"Yewi CLI"}
            </a>
            {", run the following command:"}
          </p>
          <pre class="code-block">
            <code>
              {"cargo install yewi-cli"}
            </code>
          </pre>
          <p class="doc-section-text">
            {r#"
              Or install from source:
            "#}
          </p>
          <pre class="code-block">
            <code>
            {r#"
git clone https://github.com/Emii-lia/yewi-kit.git
cd yewi-kit/cli
cargo install --path .
            "#}
            </code>
          </pre>
        </div>
        <div class="doc-subsection">
          <h2 class="doc-subsection-title">{"Create new yewi project"}</h2>
          <p class="doc-section-text">
            {r#"
              Once you have the Yewi CLI installed, you can create a new Yew project with Yewi-kit by running:
            "#}
          </p>
          <pre class="code-block">
            <code>
              {"yewi new my-yewi-app"}
            </code>
          </pre>
          <p class="doc-section-text">
            {r#"
              This will create a new directory called `my-yewi-app` with a basic Yew project structure and Yewi-kit dependencies included.
            "#}
          </p>
          <p class="doc-section-text">
            {r#"
              After creating the project, navigate into the project directory and run the following commands:
            "#}
          </p>
          <pre class="code-block">
            <code>
              {r#"
cd my-yewi-app
yarn install
yarn build
trunk serve
              "#}
            </code>
          </pre>
          <p class="doc-section-text">
            {r#"
              Make sure you have
            "#}
            <a
              href="https://classic.yarnpkg.com/lang/en/docs/install/#debian-stable"
              target="_blank"
              rel="noreferrer"
            >{"yarn"}</a>
            {" , "}
            <a
              href="https://yew.rs/docs/getting-started/introduction"
              target="_blank"
              rel="noreferrer"
            >{"WebAssembly target and trunk"}</a>
            {" installed on your system. You can now start building your application using Yewi-kit components!"}
          </p>
        </div>
        <div class="doc-subsection">
          <h2 class="doc-subsection-title">{"Manual Installation"}</h2>
          <p class="doc-section-text">
            {r#"
              If you prefer to add Yewi-kit to an existing Yew project, you can do so by mirroring your project structure with the project in the template repo
            "#}
            <a
              href="https://github.com/Emii-lia/yew-app-template"
              target="_blank"
              rel="noreferrer"
            >
              {"yew-app-template"}
            </a>
            {", along with the dependencies and configs, including tailwindcss config."}
          </p>
        </div>
        <div class="doc-subsection">
          <h2 class="doc-subsection-title">{"Add components"}</h2>
          <p class="doc-section-text">
            {r#"
              Yewi-kit components can be added to your project by running the following command:
            "#}
          </p>
          <pre class="code-block">
            <code>
              {"yewi add <component-name>"}
            </code>
          </pre>
          <p class="doc-section-text">
            {r#"
              Replace `<component-name>` with the name of the component you want to add (e.g., `button`, `card`, `modal`, etc.). This command will download the component files and
              place them in the appropriate directories within your project.
            "#}
          </p>
        </div>
      </div>
      <div class="doc-footer">
        <p class="doc-footer-text">
          {"Source: "}
          <a
            href="https://github.com/Emii-lia/yewi-kit"
            target="_blank"
            rel="noreferrer"
          >{"Yewi-kit GitHub Repository"}</a>
        </p>
      </div>
    </div>
  }
}
use yew::{function_component, html, Html};
use yew_router::prelude::Link;
use crate::app::docs::routes::DocsRoute;
use crate::components::Button;
use crate::features::{Footer, Hero, HomeCta, HomeHeader, ThemeSection};
use crate::features::home_section::{HomeSection, HomeSectionContainer, HomeSectionContent, HomeSectionDescription, HomeSectionIllustration, HomeSectionText, HomeSectionTitle};
use crate::types::Size;

#[function_component]
pub(crate) fn Home() -> Html {
  html! {
    <div class="Home">
      <HomeHeader/>
      <Hero/>
      <HomeSection>
        <HomeSectionContainer>
          <HomeSectionContent>
            <HomeSectionText>
              <HomeSectionTitle>
                {"Yew, instantly"}
              </HomeSectionTitle>
              <HomeSectionDescription>
                {r#"Get up and running in seconds. Yewi-kit sets up a complete, production-ready Yew project with everything you need to start building. One command gives you a modern, well-structured project that's ready to extend and deploy."#}
              </HomeSectionDescription>
            </HomeSectionText>
            <Link<DocsRoute>
              to={DocsRoute::Docs}
            >
              <Button size={Size::Large}>
                {"Get Started"}
              </Button>
            </Link<DocsRoute>>
          </HomeSectionContent>
          <HomeSectionIllustration
            command={Some("yewi new my-project")}
            output={vec![
              "? Select a theme:".to_string(),
              "> Slate".to_string(),
              "  Gray".to_string(),
              "  Zinc".to_string(),
              "  Neutral".to_string(),
              "  Stone".to_string(),
              "  Emerald".to_string(),
              "  Blue".to_string(),
              "[↑↓ to move, enter to select, type to filter]".to_string(),
              "Project created successfully".to_string(),
            ]}
          />
        </HomeSectionContainer>
      </HomeSection>
      <HomeSection>
        <HomeSectionContainer class="home-section-reverse">
          <HomeSectionContent>
            <HomeSectionText>
              <HomeSectionTitle>
                {"Add what you need"}
              </HomeSectionTitle>
              <HomeSectionDescription>
                {r#"Yewi lets you add UI components on demand. Each component is fully customizable and extensible. No hidden dependencies, no black boxes. Just clean, composable Rust code that you control."#}
              </HomeSectionDescription>
            </HomeSectionText>
            <Link<DocsRoute>
              to={DocsRoute::ButtonPage}
            >
              <Button size={Size::Large}>
                {"View Components"}
              </Button>
            </Link<DocsRoute>>
          </HomeSectionContent>
          <HomeSectionIllustration
            command={Some("yewi add button")}
            output={vec![
            "Adding button component...".to_string(),
            "Downloading button component from yewi-kit".to_string(),
            "Downloaded file: button.scss".to_string(),
            "Downloaded file: hooks.rs".to_string(),
            "Downloaded file: mod.rs".to_string(),
            "Downloaded file: types.rs".to_string(),
            "Component 'button' added to your project.".to_string(),
            ]}
          />
        </HomeSectionContainer>
      </HomeSection>
      <HomeSection>
        <HomeSectionContainer>
          <HomeSectionContent>
            <HomeSectionText>
              <HomeSectionTitle>
                {"Clean architecture"}
              </HomeSectionTitle>
              <HomeSectionDescription>
                {r#"Yewi projects follow modern framework patterns with a component-driven architecture. Consistent project layout, clear separation of concerns, and built-in best practices make it easy to scale your application from prototype to production."#}
              </HomeSectionDescription>
            </HomeSectionText>
            <Link<DocsRoute>
              to={DocsRoute::Installation}
            >
              <Button size={Size::Large}>
                {"Install Guide"}
              </Button>
            </Link<DocsRoute>>
          </HomeSectionContent>
          <HomeSectionIllustration
            output={vec![
              "├── src/".to_string(),
              "  ├── app/".to_string(),
              "    ├── about/".to_string(),
              "      ├── mod.rs".to_string(),
              "      ├── routes.rs".to_string(),
              "    ├── mod.rs".to_string(),
              "    ├── routes.rs".to_string(),
              "    ├── page.rs".to_string(),
              "  ├── components/".to_string(),
              "    ├── button/".to_string(),
              "      ├── mod.rs".to_string(),
              "      ├── hooks.rs".to_string(),
              "      ├── (types|data|provider).rs".to_string(),
              "      ├── button.scss".to_string(),
              "  ├── styles/".to_string(),
              "    ├── components.scss".to_string(),
              "    ├── main.scss".to_string(),
              "    ├── global.css".to_string(),
              "  ├── types/".to_string(),
              "  ├── main.rs".to_string(),
            ]}
          />
        </HomeSectionContainer>
      </HomeSection>
      <div class="last-section-container">
        <ThemeSection/>
        <HomeCta/>
      </div>
      <Footer/>
    </div>
  }
}
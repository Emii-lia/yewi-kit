use yew::{function_component, html, Html};
use crate::components::{Badge, BadgeColor, BadgeVariant};
use crate::previews::PreviewContainer;

#[function_component(BadgePreview)]
pub(crate) fn badge_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Badge"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Badges are used to display small status descriptors for UI elements."}
        </div>
        <pre class="code-block">
          <code>
{"yewi add badge"}
          </code>
        </pre>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
              <Badge label="Default" color={BadgeColor::Red}/>
              <Badge label="Default" color={BadgeColor::Blue}/>
              <Badge label="Default" color={BadgeColor::Green}/>
              <Badge label="Default" color={BadgeColor::Yellow}/>
            "#}
          >
            <Badge label="Default" color={BadgeColor::Red}/>
            <Badge label="Default" color={BadgeColor::Blue}/>
            <Badge label="Default" color={BadgeColor::Green}/>
            <Badge label="Default" color={BadgeColor::Yellow}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Plain"}
            code={r#"
              <Badge variant={BadgeVariant::Plain} label="Plain" color={BadgeColor::Red}/>
              <Badge variant={BadgeVariant::Plain} label="Plain" color={BadgeColor::Blue}/>
              <Badge variant={BadgeVariant::Plain} label="Plain" color={BadgeColor::Green}/>
              <Badge variant={BadgeVariant::Plain} label="Plain" color={BadgeColor::Yellow}/>
            "#}
          >
            <Badge variant={BadgeVariant::Plain} label="Plain" color={BadgeColor::Red}/>
            <Badge variant={BadgeVariant::Plain} label="Plain" color={BadgeColor::Blue}/>
            <Badge variant={BadgeVariant::Plain} label="Plain" color={BadgeColor::Green}/>
            <Badge variant={BadgeVariant::Plain} label="Plain" color={BadgeColor::Yellow}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Filled"}
            code={r#"
              <Badge variant={BadgeVariant::Filled} label="Filled" color={BadgeColor::Red}/>
              <Badge variant={BadgeVariant::Filled} label="Filled" color={BadgeColor::Blue}/>
              <Badge variant={BadgeVariant::Filled} label="Filled" color={BadgeColor::Green}/>
              <Badge variant={BadgeVariant::Filled} label="Filled" color={BadgeColor::Yellow}/>
            "#}
          >
            <Badge variant={BadgeVariant::Filled} label="Filled" color={BadgeColor::Red}/>
            <Badge variant={BadgeVariant::Filled} label="Filled" color={BadgeColor::Blue}/>
            <Badge variant={BadgeVariant::Filled} label="Filled" color={BadgeColor::Green}/>
            <Badge variant={BadgeVariant::Filled} label="Filled" color={BadgeColor::Yellow}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Rounded"}
            code={r#"
              <Badge label="Rounded" color={BadgeColor::Red} rounded=true />
              <Badge label="Rounded" color={BadgeColor::Blue} rounded=true />
              <Badge label="Rounded" color={BadgeColor::Green} rounded=true />
              <Badge label="Rounded" color={BadgeColor::Yellow} rounded=true />
            "#}
          >
            <Badge label="Rounded" color={BadgeColor::Red} rounded=true />
            <Badge label="Rounded" color={BadgeColor::Blue} rounded=true />
            <Badge label="Rounded" color={BadgeColor::Green} rounded=true />
            <Badge label="Rounded" color={BadgeColor::Yellow} rounded=true />
          </PreviewContainer>
          <PreviewContainer
            title={"Bordered"}
            code={r#"
              <Badge label="Bordered" color={BadgeColor::Red} with_border=true/>
              <Badge label="Bordered" color={BadgeColor::Blue} with_border=true/>
              <Badge label="Bordered" color={BadgeColor::Green} with_border=true/>
              <Badge label="Bordered" color={BadgeColor::Yellow} with_border=true/>
            "#}
          >
            <Badge label="Bordered" color={BadgeColor::Red} with_border=true/>
            <Badge label="Bordered" color={BadgeColor::Blue} with_border=true/>
            <Badge label="Bordered" color={BadgeColor::Green} with_border=true/>
            <Badge label="Bordered" color={BadgeColor::Yellow} with_border=true/>
          </PreviewContainer>
          <PreviewContainer
            title={"Colours"}
            code={r#"
              <Badge label="Orange" color={BadgeColor::Orange}/>
              <Badge label="Teal" color={BadgeColor::Teal}/>
              <Badge label="Purple" color={BadgeColor::Purple}/>
              <Badge label="Zinc" color={BadgeColor::Zinc}/>
              <Badge label="Sky" color={BadgeColor::Sky}/>
            "#}
          >
            <Badge label="Orange" color={BadgeColor::Orange}/>
            <Badge label="Teal" color={BadgeColor::Teal}/>
            <Badge label="Purple" color={BadgeColor::Purple}/>
            <Badge label="Zinc" color={BadgeColor::Zinc}/>
            <Badge label="Sky" color={BadgeColor::Sky}/>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
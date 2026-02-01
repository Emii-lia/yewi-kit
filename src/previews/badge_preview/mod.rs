use yew::{function_component, html, Html};
use yew_icons::IconData;
use crate::components::{Badge, BadgeVariant};
use crate::previews::PreviewContainer;
use crate::types::Color;

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
          {"Display small status descriptors for UI elements."}
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
    <Badge label="Default"/>
    <Badge label="Default" color={Color::Blue}/>
    <Badge label="Default" color={Color::Green}/>
    <Badge label="Default" color={Color::Red}/>
            "#}
          >
            <Badge label="Default"/>
            <Badge label="Default" color={Color::Blue}/>
            <Badge label="Default" color={Color::Green}/>
            <Badge label="Default" color={Color::Red}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Plain"}
            code={r#"
    <Badge variant={BadgeVariant::Plain} label="Plain"/>
    <Badge variant={BadgeVariant::Plain} label="Plain" color={Color::Blue}/>
    <Badge variant={BadgeVariant::Plain} label="Plain" color={Color::Green}/>
    <Badge variant={BadgeVariant::Plain} label="Plain" color={Color::Red}/>
            "#}
          >
            <Badge variant={BadgeVariant::Plain} label="Plain"/>
            <Badge variant={BadgeVariant::Plain} label="Plain" color={Color::Blue}/>
            <Badge variant={BadgeVariant::Plain} label="Plain" color={Color::Green}/>
            <Badge variant={BadgeVariant::Plain} label="Plain" color={Color::Red}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Filled"}
            code={r#"
    <Badge variant={BadgeVariant::Filled} label="Filled"/>
    <Badge variant={BadgeVariant::Filled} label="Filled" color={Color::Blue}/>
    <Badge variant={BadgeVariant::Filled} label="Filled" color={Color::Green}/>
    <Badge variant={BadgeVariant::Filled} label="Filled" color={Color::Red}/>
            "#}
          >
            <Badge variant={BadgeVariant::Filled} label="Filled"/>
            <Badge variant={BadgeVariant::Filled} label="Filled" color={Color::Blue}/>
            <Badge variant={BadgeVariant::Filled} label="Filled" color={Color::Green}/>
            <Badge variant={BadgeVariant::Filled} label="Filled" color={Color::Red}/>
          </PreviewContainer>
          <PreviewContainer
            title={"With Icon"}
            code={r#"
    <Badge label="Stop" color={Color::Red} icon={IconData::LUCIDE_STOP_CIRCLE} />
    <Badge variant={BadgeVariant::Plain} label="Info" color={Color::Blue} icon={IconData::LUCIDE_INFO}/>
    <Badge variant={BadgeVariant::Filled} label="Success" color={Color::Green} icon={IconData::LUCIDE_CHECK}/>
            "#}
          >
            <Badge label="Stop" color={Color::Red} icon={IconData::LUCIDE_KEY} />
            <Badge variant={BadgeVariant::Plain} label="Info" color={Color::Blue} icon={IconData::LUCIDE_INFO}/>
            <Badge variant={BadgeVariant::Filled} label="Success" color={Color::Green} icon={IconData::LUCIDE_CHECK}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Rounded"}
            code={r#"
    <Badge label="Rounded" rounded=true />
    <Badge label="Rounded" color={Color::Blue} rounded=true />
    <Badge label="Rounded" color={Color::Green} rounded=true />
    <Badge label="Rounded" color={Color::Red} rounded=true />
            "#}
          >
            <Badge label="Rounded" rounded=true />
            <Badge label="Rounded" color={Color::Blue} rounded=true />
            <Badge label="Rounded" color={Color::Green} rounded=true />
            <Badge label="Rounded" color={Color::Red} rounded=true />
          </PreviewContainer>
          <PreviewContainer
            title={"Bordered"}
            code={r#"
    <Badge label="Bordered" with_border=true/>
    <Badge label="Bordered" color={Color::Blue} with_border=true/>
    <Badge label="Bordered" color={Color::Green} with_border=true/>
    <Badge label="Bordered" color={Color::Red} with_border=true/>
            "#}
          >
            <Badge label="Bordered" with_border=true/>
            <Badge label="Bordered" color={Color::Blue} with_border=true/>
            <Badge label="Bordered" color={Color::Green} with_border=true/>
            <Badge label="Bordered" color={Color::Red} with_border=true/>
          </PreviewContainer>
          <PreviewContainer
            title={"Colours"}
            code={r#"
    <Badge label="Orange" color={Color::Orange}/>
    <Badge label="Teal" color={Color::Teal}/>
    <Badge label="Purple" color={Color::Purple}/>
    <Badge label="Zinc" color={Color::Zinc}/>
    <Badge label="Sky" color={Color::Sky}/>
            "#}
          >
            <Badge label="Orange" color={Color::Orange}/>
            <Badge label="Teal" color={Color::Teal}/>
            <Badge label="Purple" color={Color::Purple}/>
            <Badge label="Zinc" color={Color::Gray}/>
            <Badge label="Sky" color={Color::Sky}/>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
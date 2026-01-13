use yew::{function_component, html, Html};
use crate::components::{Button, ButtonVariant};
use crate::previews::PreviewContainer;
use crate::types::{Size};

#[function_component(ButtonPreview)]
pub(crate) fn button_preview () -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Button"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display clickable buttons with various styles and sizes."}
        </div>
        <pre class="code-block">
          <code>
{"yewi add button"}
          </code>
        </pre>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title="Primary"
            code={r#"
              <Button size={Size::Small}>
                {"Primary"}
              </Button>
              <Button size={Size::Medium}>
                {"Primary"}
              </Button>
              <Button size={Size::Large}>
                {"Primary"}
              </Button>
            "#}
          >
            <Button size={Size::Small}>
              {"Primary"}
            </Button>
            <Button size={Size::Medium}>
              {"Primary"}
            </Button>
            <Button size={Size::Large}>
              {"Primary"}
            </Button>
          </PreviewContainer>
          <PreviewContainer
            title="Secondary"
            code={r#"
              <Button size={Size::Small} variant={ButtonVariant::Secondary} >
                {"Secondary"}
              </Button>
              <Button size={Size::Medium} variant={ButtonVariant::Secondary} >
                {"Secondary"}
              </Button>
              <Button size={Size::Large} variant={ButtonVariant::Secondary} >
                {"Secondary"}
              </Button>
            "#}
          >
            <Button size={Size::Small} variant={ButtonVariant::Secondary} >
              {"Secondary"}
            </Button>
            <Button size={Size::Medium} variant={ButtonVariant::Secondary} >
              {"Secondary"}
            </Button>
            <Button size={Size::Large} variant={ButtonVariant::Secondary} >
              {"Secondary"}
            </Button>
          </PreviewContainer>
          <PreviewContainer
            title="Tertiary"
            code={r#"
              <Button size={Size::Small} variant={ButtonVariant::Tertiary} >
                {"Tertiary"}
              </Button>
              <Button size={Size::Medium} variant={ButtonVariant::Tertiary} >
                {"Tertiary"}
              </Button>
              <Button size={Size::Large} variant={ButtonVariant::Tertiary} >
                {"Tertiary"}
              </Button>
            "#}
          >
            <Button size={Size::Small} variant={ButtonVariant::Tertiary} >
              {"Tertiary"}
            </Button>
            <Button size={Size::Medium} variant={ButtonVariant::Tertiary} >
              {"Tertiary"}
            </Button>
            <Button size={Size::Large} variant={ButtonVariant::Tertiary} >
              {"Tertiary"}
            </Button>
          </PreviewContainer>
          <PreviewContainer
            title="Success"
            code={r#"
              <Button size={Size::Small} variant={ButtonVariant::Success} >
                {"Success"}
              </Button>
              <Button size={Size::Medium} variant={ButtonVariant::Success} >
                {"Success"}
              </Button>
              <Button size={Size::Large} variant={ButtonVariant::Success} >
                {"Success"}
              </Button>
            "#}
          >
            <Button size={Size::Small} variant={ButtonVariant::Success} >
              {"Success"}
            </Button>
            <Button size={Size::Medium} variant={ButtonVariant::Success} >
              {"Success"}
            </Button>
            <Button size={Size::Large} variant={ButtonVariant::Success} >
              {"Success"}
            </Button>
          </PreviewContainer>
          <PreviewContainer
            title="Danger"
            code={r#"
              <Button size={Size::Small} variant={ButtonVariant::Danger} >
                {"Danger"}
              </Button>
              <Button size={Size::Medium} variant={ButtonVariant::Danger} >
                {"Danger"}
              </Button>
              <Button size={Size::Large} variant={ButtonVariant::Danger} >
                {"Danger"}
              </Button>
            "#}
          >
            <Button size={Size::Small} variant={ButtonVariant::Danger} >
              {"Danger"}
            </Button>
            <Button size={Size::Medium} variant={ButtonVariant::Danger} >
              {"Danger"}
            </Button>
            <Button size={Size::Large} variant={ButtonVariant::Danger} >
              {"Danger"}
            </Button>
          </PreviewContainer>
          <PreviewContainer
            title="As Link"
            code={r#"
              <Button href="https://github.com/Emii-lia" >
                {"Link"}
              </Button>
            "#}
          >
            <Button href="https://github.com/Emii-lia" >
              {"Link"}
            </Button>
          </PreviewContainer>
          <PreviewContainer
            title="Disabled"
            code={r#"
              <Button disabled=true >
                {"Disabled"}
              </Button>
            "#}
          >
            <Button disabled=true >
              {"Disabled"}
            </Button>
          </PreviewContainer>
          <PreviewContainer
            title="Loading"
            code={r#"
              <Button is_loading=true >
                {"Loading"}
              </Button>
            "#}
          >
            <Button is_loading=true >
              {"Loading"}
            </Button>
          </PreviewContainer>
          <PreviewContainer
            title="Custom Class"
            code={r#"
              <Button class="!bg-slate-500" >
                {"Custom"}
              </Button>
            "#}
          >
            <Button class="!bg-slate-500" >
              {"Custom"}
            </Button>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
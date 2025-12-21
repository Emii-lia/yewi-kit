use yew::{function_component, html, Html};
use crate::components::Button;
use crate::previews::PreviewContainer;
use crate::types::{ButtonVariant, Size};

#[function_component(ButtonPreview)]
pub(crate) fn button_preview () -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Button"}</h1>
      <div class="preview-list">
        <PreviewContainer title="Primary">
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
        <PreviewContainer title="Secondary">
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
        <PreviewContainer title="Tertiary">
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
        <PreviewContainer title="Success">
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
        <PreviewContainer title="Danger">
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
        <PreviewContainer title="As Link">
          <Button href="https://github.com/Emii-lia" >
            {"Link"}
          </Button>
        </PreviewContainer>
        <PreviewContainer title="Disabled">
          <Button disabled=true >
            {"Disabled"}
          </Button>
        </PreviewContainer>
        <PreviewContainer title="Loading">
          <Button is_loading=true >
            {"Loading"}
          </Button>
        </PreviewContainer>
        <PreviewContainer title="Custom Class">
          <Button class="!bg-slate-500" >
            {"Custom"}
          </Button>
        </PreviewContainer>
      </div>
    </div>
  }
}
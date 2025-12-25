use yew::{function_component, html, Html};
use crate::components::Textarea;
use crate::previews::PreviewContainer;

#[function_component(TextareaPreview)]
pub(crate) fn textarea_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Textarea"}</h1>
      <div class="preview-list">
        <PreviewContainer title={"Default"}>
          <Textarea placeholder="Type here..." rows={"4"} />
        </PreviewContainer>
        <PreviewContainer title={"Disabled"}>
          <Textarea placeholder="Cannot type here..." rows={"4"} disabled=true />
        </PreviewContainer>
        <PreviewContainer title={"With label"}>
          <Textarea
            placeholder="Type here..."
            rows={"4"}
            label={"Description"} 
          />
        </PreviewContainer>
      </div>
    </div>
  }
}
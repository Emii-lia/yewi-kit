use yew::{function_component, html, Html};
use crate::components::Divider;
use crate::previews::PreviewContainer;

#[function_component(DividerPreview)]
pub(crate) fn divider_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Divider"}</h1>
      <div class="preview-list">
        <PreviewContainer title="Horizontal">
          <Divider/>
          <Divider>{"OR"}</Divider>
        </PreviewContainer>
        <PreviewContainer title="Vertical">
          <Divider vertical=true/>
          <Divider vertical=true>{"OR"}</Divider>
        </PreviewContainer>
      </div>
    </div>
  }
}
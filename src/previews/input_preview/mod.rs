use yew::{function_component, html, Html};
use crate::components::Input;
use crate::previews::PreviewContainer;
use crate::types::Size;

#[function_component(InputPreview)]
pub(crate) fn input_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Input"}</h1>
      <div class="preview-list">
        <PreviewContainer title={"Default"}>
          <Input placeholder={"Type here..."} input_size={Size::Small}/>
          <Input placeholder={"Type here..."} input_size={Size::Medium}/>
          <Input placeholder={"Type here..."} input_size={Size::Large}/>
        </PreviewContainer>
        <PreviewContainer title={"Disabled"}>
          <Input placeholder={"Cannot type here..."} input_size={Size::Small} disabled=true/>
          <Input placeholder={"Cannot type here..."} input_size={Size::Medium} disabled=true/>
          <Input placeholder={"Cannot type here..."} input_size={Size::Large} disabled=true/>
        </PreviewContainer>
      </div>
    </div>
  }
}
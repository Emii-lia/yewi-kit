use yew::{function_component, html, Html};
use crate::components::{Button, FileInput, FileInputType};
use crate::previews::PreviewContainer;

#[function_component(FileInputPreviews)]
pub(crate) fn file_input_previews() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"File Input"}</h1>
      <div class="preview-list">
        <PreviewContainer title={"Default"}>
          <FileInput/>
        </PreviewContainer>
        <PreviewContainer title={"Drag and drop"}>
          <FileInput r#type={FileInputType::DnD}/>
        </PreviewContainer>
        <PreviewContainer title={"Button"}>
          <FileInput r#type={FileInputType::Button}>
            {"Upload files"}
          </FileInput>
        </PreviewContainer>
        <PreviewContainer title={"Disabled"}>
          <FileInput disabled=true/>
          <FileInput r#type={FileInputType::DnD} disabled=true/>
          <FileInput r#type={FileInputType::Button} disabled=true>
            {"Upload files"}
          </FileInput>
        </PreviewContainer>
      </div>
    </div>
  }
}
use yew::{function_component, html, Html};
use crate::components::{ FileInput, FileInputType};
use crate::previews::PreviewContainer;

#[function_component(FileInputPreviews)]
pub(crate) fn file_input_previews() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"File Input"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display file upload inputs with different variants."}
        </div>
        <pre class="code-block">
          <code>
    {"yewi add button file_input"}
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
    <FileInput/>
            "#}
          >
            <FileInput/>
          </PreviewContainer>
          <PreviewContainer
            title={"Drag and drop"}
            code={r#"
    <FileInput r#type={FileInputType::DnD}/>
            "#}
          >
            <FileInput r#type={FileInputType::DnD}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Button"}
            code={r#"
    <FileInput r#type={FileInputType::Button}>
      {"Upload files"}
    </FileInput>
            "#}
          >
            <FileInput r#type={FileInputType::Button}>
              {"Upload files"}
            </FileInput>
          </PreviewContainer>
          <PreviewContainer
            title={"Disabled"}
            code={r#"
    <FileInput disabled=true/>
    <FileInput r#type={FileInputType::DnD} disabled=true/>
    <FileInput r#type={FileInputType::Button} disabled=true>
      {"Upload files"}
    </FileInput>
            "#}
          >
            <FileInput disabled=true/>
            <FileInput r#type={FileInputType::DnD} disabled=true/>
            <FileInput r#type={FileInputType::Button} disabled=true>
              {"Upload files"}
            </FileInput>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
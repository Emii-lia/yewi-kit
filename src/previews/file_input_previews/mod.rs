mod data;

use yew::{function_component, html, Html};
use crate::components::{CodePreview, FileInput, FileInputType};
use crate::features::prop_table::PropTable;
use crate::previews::file_input_previews::data::get_props;
use crate::previews::PreviewContainer;

#[function_component(FileInputPreviews)]
pub(crate) fn file_input_previews() -> Html {
  let props = get_props();
  
  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">{"File Input"}</h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display file upload inputs with different variants."}
        </div>
        <CodePreview code={"yewi add file_input button"}/>
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
      <PropTable props={props}/>
    </div>
  }
}
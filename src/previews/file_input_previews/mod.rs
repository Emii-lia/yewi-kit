mod data;

use yew::{component, html, Html};
use yew_icons::IconData;
use crate::components::button::ButtonVariant;
use crate::components::code_preview::CodePreview;
use crate::components::file_input::{FileInput, FileInputType};
use crate::features::prop_table::PropTable;
use crate::previews::file_input_previews::data::get_props;
use crate::previews::PreviewContainer;
use crate::types::Size;

#[component(FileInputPreviews)]
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
        <CodePreview code={"yewi add file_input"}/>
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
    <FileInput r#type={FileInputType::button()}>
      {"Upload files"}
    </FileInput>
    <FileInput r#type={FileInputType::button().with_variant(ButtonVariant::Secondary)}>
      {"Upload files"}
    </FileInput>
    <FileInput r#type={FileInputType::button().with_icon(Some(IconData::LUCIDE_UPLOAD))}>
      {"Upload files"}
    </FileInput>
    <FileInput r#type={FileInputType::button().with_variant(ButtonVariant::Secondary).with_size(Size::Large)}>
      {"Upload files"}
    </FileInput>
            "#}
          >
            <FileInput r#type={FileInputType::button()}>
              {"Upload files"}
            </FileInput>
            <FileInput r#type={FileInputType::button().with_variant(ButtonVariant::Secondary)}>
              {"Upload files"}
            </FileInput>
            <FileInput r#type={FileInputType::button().with_icon(Some(IconData::LUCIDE_UPLOAD))}>
              {"Upload files"}
            </FileInput>
            <FileInput r#type={FileInputType::button().with_variant(ButtonVariant::Secondary).with_size(Size::Large)}>
              {"Upload files"}
            </FileInput>
          </PreviewContainer>
          <PreviewContainer
            title={"Disabled"}
            code={r#"
    <FileInput disabled=true/>
    <FileInput r#type={FileInputType::DnD} disabled=true/>
    <FileInput r#type={FileInputType::button()} disabled=true>
      {"Upload files"}
    </FileInput>
            "#}
          >
            <FileInput disabled=true/>
            <FileInput r#type={FileInputType::DnD} disabled=true/>
            <FileInput r#type={FileInputType::button()} disabled=true>
              {"Upload files"}
            </FileInput>
          </PreviewContainer>
        </div>
      </div>
      <PropTable props={props}/>
    </div>
  }
}
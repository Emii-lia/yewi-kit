pub mod data;
pub mod hooks;

use yew::{component, html, Callback, Html};
use crate::components::code_preview::CodePreview;
use crate::components::file_input::{FileInput, FileInputType};
use crate::components::file_preview::{FilePreview};
use crate::components::file_preview::types::FileValue;
use crate::features::prop_table::PropTable;
use crate::previews::file_preview_preview::data::get_props;
use crate::previews::file_preview_preview::hooks::use_file_preview_preview;
use crate::previews::PreviewContainer;

#[component(FilePreviewPreview)]
pub fn file_preview_preview() -> Html {
  let (files, onchange, onremove) = use_file_preview_preview();
  let props = get_props();

  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">{"Checkbox"}</h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display checkboxes with various styles, variants and sizes."}
        </div>
        <CodePreview code={"yewi add file_preview"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
            "#}
          >
            <div class="file-preview-field">
              <FileInput onchange={onchange} multiple=true r#type={FileInputType::button()}>
                {"Add files"}
              </FileInput>
              <div class="file-preview-list">
                {for files.iter().enumerate().map(|(i, file)| {
                  html! {
                    <FilePreview
                      file={FileValue::file(file.clone())}
                      onremove={{
                        let onremove = onremove.clone();
                        Callback::from(move |_| {
                          onremove.emit(i as u32);
                        })
                      }}
                    />
                  }
                })}
              </div>
            </div>
          </PreviewContainer>
        </div>
      </div>
      <PropTable props={props}/>
    </div>
  }
}
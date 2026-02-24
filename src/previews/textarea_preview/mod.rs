mod data;

use yew::{function_component, html, Html};
use crate::components::{CodePreview, Textarea};
use crate::features::prop_table::PropTable;
use crate::previews::PreviewContainer;
use crate::previews::textarea_preview::data::get_props;

#[function_component(TextareaPreview)]
pub(crate) fn textarea_preview() -> Html {
  let props = get_props();
  
  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">{"Textarea"}</h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Use the Textarea component to allow users to input multi-line text."}
        </div>
        <CodePreview code={"yewi add textarea"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
    <Textarea placeholder="Type here..." rows={"4"} />
            "#}
          >
            <Textarea placeholder="Type here..." rows={"4"} />
          </PreviewContainer>
          <PreviewContainer
            title={"Disabled"}
            code={r#"
    <Textarea placeholder="Cannot type here..." rows={"4"} disabled=true />
            "#}
          >
            <Textarea placeholder="Cannot type here..." rows={"4"} disabled=true />
          </PreviewContainer>
          <PreviewContainer
            title={"With label"}
            code={r#"
    <Textarea
      placeholder="Type here..."
      rows={"4"}
      label={"Description"}
    />
            "#}
          >
            <Textarea
              placeholder="Type here..."
              rows={"4"}
              label={"Description"}
            />
          </PreviewContainer>
          <PreviewContainer
            title={"With error"}
            code={r#"
    <Textarea placeholder="Type here..." rows={"4"} errors={vec!["Invalid input".to_string()]}/>
            "#}
          >
            <Textarea placeholder="Type here..." rows={"4"} errors={vec!["Invalid input".to_string()]}/>
          </PreviewContainer>
        </div>
      </div>
      <PropTable props={props} />
    </div>
  }
}
use yew::{function_component, html, Html};
use crate::components::Textarea;
use crate::previews::PreviewContainer;

#[function_component(TextareaPreview)]
pub(crate) fn textarea_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Textarea"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Use the Textarea component to allow users to input multi-line text."}
        </div>
        <pre class="code-block">
          <code>
{"yewi add textarea"}
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
    </div>
  }
}
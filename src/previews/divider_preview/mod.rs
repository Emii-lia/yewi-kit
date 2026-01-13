use yew::{function_component, html, Html};
use crate::components::Divider;
use crate::previews::PreviewContainer;

#[function_component(DividerPreview)]
pub(crate) fn divider_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Divider"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Divide content with horizontal or vertical dividers."}
        </div>
        <pre class="code-block">
          <code>
{"yewi add divider"}
          </code>
        </pre>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title="Horizontal"
            code={r#"
              <Divider/>
              <Divider>{"OR"}</Divider>
            "#}
          >
            <Divider/>
            <Divider>{"OR"}</Divider>
          </PreviewContainer>
          <PreviewContainer
            title="Vertical"
            code={r#"
              <Divider vertical=true/>
              <Divider vertical=true>{"OR"}</Divider>
            "#}
          >
            <Divider vertical=true/>
            <Divider vertical=true>{"OR"}</Divider>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
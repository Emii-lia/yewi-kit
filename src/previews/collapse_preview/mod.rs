use yew::{function_component, html, Html};
use crate::components::{Collapse, CollapseVariant};
use crate::previews::PreviewContainer;

#[function_component(CollapsePreview)]
pub(crate) fn collapse_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Collapse"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Create expandable and collapsible sections of content."}
        </div>
        <pre class="code-block">
          <code>
{"yewi add collapse"}
          </code>
        </pre>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Focus Variant"}
            code={r#"
              <Collapse summary={"See more details"}>
                <div class="ex-collapse-content">
                  <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
                  <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
                </div>
              </Collapse>
            "#}
          >
            <Collapse summary={"See more details"}>
              <div class="ex-collapse-content">
                <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
                <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
              </div>
            </Collapse>
          </PreviewContainer>
          <PreviewContainer
            title={"Toggle Variant"}
            code={r#"
              <Collapse summary={"See more details"} variant={CollapseVariant::Toggle}>
                <div class="ex-collapse-content">
                  <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
                  <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
                </div>
              </Collapse>
            "#}
          >
            <Collapse summary={"See more details"} variant={CollapseVariant::Toggle}>
              <div class="ex-collapse-content">
                <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
                <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
              </div>
            </Collapse>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
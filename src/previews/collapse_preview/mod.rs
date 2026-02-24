mod data;

use yew::{function_component, html, Html};
use crate::components::{CodePreview, Collapse, CollapseVariant};
use crate::features::prop_table::PropTable;
use crate::previews::collapse_preview::data::get_props;
use crate::previews::PreviewContainer;

#[function_component(CollapsePreview)]
pub(crate) fn collapse_preview() -> Html {
  let props = get_props();

  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">{"Collapse"}</h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Create expandable and collapsible sections of content."}
        </div>
        <CodePreview code={"yewi add collapse"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Focus"}
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
            title={"Toggle"}
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
      <PropTable props={props}/>
    </div>
  }
}
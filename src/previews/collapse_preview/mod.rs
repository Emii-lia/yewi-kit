mod data;

use yew::{component, html, Html};
use yew_icons::IconData;
use crate::components::code_preview::CodePreview;
use crate::components::collapse::{Collapse, CollapseContent, CollapseIndicator, CollapseTrigger, CollapseVariant};
use crate::features::component_table::ComponentTable;
use crate::features::prop_table::PropTable;
use crate::previews::collapse_preview::data::{get_components, get_props};
use crate::previews::PreviewContainer;

#[component(CollapsePreview)]
pub(crate) fn collapse_preview() -> Html {
  let props = get_props();
  let components = get_components();

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
    <Collapse>
      <CollapseTrigger>
        {"See more details"}
      </CollapseTrigger>
      <CollapseContent>
        <div class="ex-collapse-content">
          <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
          <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
        </div>
      </CollapseContent>
    </Collapse>
            "#}
          >
            <Collapse>
              <CollapseTrigger>
                {"See more details"}
              </CollapseTrigger>
              <CollapseContent>
                <div class="ex-collapse-content">
                  <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
                  <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
                </div>
              </CollapseContent>
            </Collapse>
          </PreviewContainer>
          <PreviewContainer
            title={"Toggle"}
            code={r#"
    <Collapse variant={CollapseVariant::toggle()}>
      <CollapseTrigger>
        {"See more details"}
      </CollapseTrigger>
      <CollapseContent>
        <div class="ex-collapse-content">
          <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
          <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
        </div>
      </CollapseContent>
    </Collapse>
    <Collapse variant={CollapseVariant::Toggle(true)}>
      <CollapseTrigger>
        {"See more details"}
      </CollapseTrigger>
      <CollapseContent>
        <div class="ex-collapse-content">
          <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
          <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
        </div>
      </CollapseContent>
    </Collapse>
            "#}
          >
            <Collapse variant={CollapseVariant::toggle()}>
              <CollapseTrigger>
                {"See more details"}
              </CollapseTrigger>
              <CollapseContent>
                <div class="ex-collapse-content">
                  <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
                  <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
                </div>
              </CollapseContent>
            </Collapse>
            <Collapse variant={CollapseVariant::Toggle(true)}>
              <CollapseTrigger>
                {"See more details"}
              </CollapseTrigger>
              <CollapseContent>
                <div class="ex-collapse-content">
                  <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
                  <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
                </div>
              </CollapseContent>
            </Collapse>
          </PreviewContainer>
          <PreviewContainer
            title={"With icon"}
            code={r#"
    <Collapse>
      <CollapseTrigger>
        {"See more details"}
      </CollapseTrigger  icon={Some(IconData::LUCIDE_MORE_VERTICAL)}>
      <CollapseContent>
        <div class="ex-collapse-content">
          <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
          <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
        </div>
      </CollapseContent>
    </Collapse>
            "#}
          >
              <Collapse>
                <CollapseTrigger icon={Some(IconData::LUCIDE_MORE_VERTICAL)}>
                  {"See more details"}
                </CollapseTrigger>
                <CollapseContent>
                  <div class="ex-collapse-content">
                    <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
                    <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
                  </div>
                </CollapseContent>
              </Collapse>
          </PreviewContainer>
          <PreviewContainer
            title={"Indicator"}
            code={r#"
    <Collapse>
      <CollapseTrigger indicator={CollapseIndicator::Chevron}>
        {"See more details"}
      </CollapseTrigger>
      <CollapseContent>
        <div class="ex-collapse-content">
          <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
          <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
        </div>
      </CollapseContent>
    </Collapse>
            "#}
          >
            <Collapse>
              <CollapseTrigger indicator={CollapseIndicator::Chevron}>
                {"See more details"}
              </CollapseTrigger>
              <CollapseContent>
                <div class="ex-collapse-content">
                  <h2 class="ex-collapse-title">{ "This is a Collapse" }</h2>
                  <p class="ex-collapse-description">{ "Collapse toggles its content vibisi- vilisibili- vibilisi-... visibilility" }</p>
                </div>
              </CollapseContent>
            </Collapse>
          </PreviewContainer>

        </div>
      </div>
      <ComponentTable components={components}/>
      <PropTable props={props}/>
    </div>
  }
}
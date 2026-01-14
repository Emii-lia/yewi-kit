use yew::{function_component, html, Html};
use crate::components::Radio;
use crate::previews::PreviewContainer;
use crate::types::{Color, Size};

#[function_component(RadioPreview)]
pub(crate) fn radio_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Radio"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Select one option from a set."}
        </div>
        <pre class="code-block">
          <code>
    {"yewi add radio"}
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
    <Radio name={"radio_default"} id={"radio_default_1"} size={Size::Small}/>
    <Radio name={"radio_default"} id={"radio_default_2"} size={Size::Medium}/>
    <Radio name={"radio_default"} id={"radio_default_3"} size={Size::Large}/>
            "#}
          >
            <Radio name={"radio_default"} id={"radio_default_1"} size={Size::Small}/>
            <Radio name={"radio_default"} id={"radio_default_2"} size={Size::Medium}/>
            <Radio name={"radio_default"} id={"radio_default_3"} size={Size::Large}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Disabled"}
            code={r#"
    <Radio name={"radio_disabled"} id={"radio_disabled_1"} size={Size::Small} disabled=true/>
    <Radio name={"radio_disabled"} id={"radio_disabled_2"} size={Size::Medium} disabled=true/>
    <Radio name={"radio_disabled"} id={"radio_disabled_3"} size={Size::Large} disabled=true/>
            "#}
          >
            <Radio name={"radio_disabled"} id={"radio_disabled_1"} size={Size::Small} disabled=true/>
            <Radio name={"radio_disabled"} id={"radio_disabled_2"} size={Size::Medium} disabled=true/>
            <Radio name={"radio_disabled"} id={"radio_disabled_3"} size={Size::Large} disabled=true/>
          </PreviewContainer>
          <PreviewContainer
            title={"Colours"}
            code={r#"
    <Radio name={"radio_coloured"} id={"radio_coloured_1"}  color={Color::Yellow}/>
    <Radio name={"radio_coloured"} id={"radio_coloured_2"} color={Color::Rose}/>
    <Radio name={"radio_coloured"} id={"radio_coloured_3"} color={Color::Teal}/>
    <Radio name={"radio_coloured"} id={"radio_coloured_4"} color={Color::Amber}/>
    <Radio name={"radio_coloured"} id={"radio_coloured_5"} color={Color::Fuchsia}/>
            "#}
          >
            <Radio name={"radio_coloured"} id={"radio_coloured_1"}  color={Color::Yellow}/>
            <Radio name={"radio_coloured"} id={"radio_coloured_2"} color={Color::Rose}/>
            <Radio name={"radio_coloured"} id={"radio_coloured_3"} color={Color::Teal}/>
            <Radio name={"radio_coloured"} id={"radio_coloured_4"} color={Color::Amber}/>
            <Radio name={"radio_coloured"} id={"radio_coloured_5"} color={Color::Fuchsia}/>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
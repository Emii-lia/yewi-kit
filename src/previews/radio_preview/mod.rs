use yew::{function_component, html, Html};
use crate::components::Radio;
use crate::previews::PreviewContainer;
use crate::types::{Color, Size};

#[function_component(RadioPreview)]
pub(crate) fn radio_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Radio"}</h1>
      <div class="preview-list">
        <PreviewContainer title={"Default"}>
          <Radio name={"radio_default"} id={"radio_default_1"} size={Size::Small}/>
          <Radio name={"radio_default"} id={"radio_default_2"} size={Size::Medium}/>
          <Radio name={"radio_default"} id={"radio_default_3"} size={Size::Large}/>
        </PreviewContainer>
        <PreviewContainer title={"Disabled"}>
          <Radio name={"radio_disabled"} id={"radio_disabled_1"} size={Size::Small} disabled=true/>
          <Radio name={"radio_disabled"} id={"radio_disabled_2"} size={Size::Medium} disabled=true/>
          <Radio name={"radio_disabled"} id={"radio_disabled_3"} size={Size::Large} disabled=true/>
        </PreviewContainer>
        <PreviewContainer title={"Colours"}>
          <Radio name={"radio_coloured"} id={"radio_coloured_1"}  color={Color::Yellow}/>
          <Radio name={"radio_coloured"} id={"radio_coloured_2"} color={Color::Rose}/>
          <Radio name={"radio_coloured"} id={"radio_coloured_3"} color={Color::Teal}/>
          <Radio name={"radio_coloured"} id={"radio_coloured_4"} color={Color::Amber}/>
          <Radio name={"radio_coloured"} id={"radio_coloured_5"} color={Color::Fuchsia}/>
        </PreviewContainer>
      </div>
    </div>
  }
}
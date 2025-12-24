mod data;

use yew::{function_component, html, Html};
use crate::components::Select;
use crate::previews::PreviewContainer;
use crate::previews::select_preview::data::options;
use crate::types::Size;

#[function_component(SelectPreview)]
pub(crate) fn select_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">
        {"Select"}
      </h1>
      <div class="preview-list">
        <PreviewContainer title={"Default"}>
          <Select
            options={options()}
            value={""}
            label={"Select an option"}
            size={Size::Small}
          />
          <Select
            options={options()}
            value={""}
            label={"Select an option"}
            size={Size::Medium}
          />
          <Select
            options={options()}
            value={""}
            label={"Select an option"}
            size={Size::Large}
          />
        </PreviewContainer>
        <PreviewContainer title={"Disabled"}>
          <Select
            options={options()}
            value={""}
            label={"Cannot select an option"}
            size={Size::Small}
            disabled=true
          />
          <Select
            options={options()}
            value={""}
            label={"Cannot select an option"}
            size={Size::Medium}
            disabled=true
          />
          <Select
            options={options()}
            value={""}
            label={"Cannot select an option"}
            size={Size::Large}
            disabled=true
          />
        </PreviewContainer>

      </div>
    </div>
  }
}
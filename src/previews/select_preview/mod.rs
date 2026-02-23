mod data;

use yew::{function_component, html, Html};
use crate::components::{CodePreview, Select};
use crate::features::prop_table::PropTable;
use crate::previews::PreviewContainer;
use crate::previews::select_preview::data::{get_props, options};
use crate::types::Size;

#[function_component(SelectPreview)]
pub(crate) fn select_preview() -> Html {
  let props = get_props();
  
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">
        {"Select"}
      </h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Choose an option from a dropdown menu."}
        </div>
        <CodePreview code={"yewi add select"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
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
            "#}
          >
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
          <PreviewContainer
            title={"Disabled"}
            code={r#"
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
            "#}
          >
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
          <PreviewContainer
            title={"With error"}
            code={r#"
    <Select
      options={options()}
      value={""}
      label={"Select an option"}
      size={Size::Small}
      errors={vec!["Invalid selection".to_string()]}
    />
    <Select
      options={options()}
      value={""}
      label={"Select an option"}
      size={Size::Medium}
      errors={vec!["Invalid selection".to_string()]}
    />
    <Select
      options={options()}
      value={""}
      label={"Select an option"}
      size={Size::Large}
      errors={vec!["Invalid selection".to_string()]}
    />
            "#}
          >
            <Select
              options={options()}
              value={""}
              label={"Select an option"}
              size={Size::Small}
              errors={vec!["Invalid selection".to_string()]}
            />
            <Select
              options={options()}
              value={""}
              label={"Select an option"}
              size={Size::Medium}
              errors={vec!["Invalid selection".to_string()]}
            />
            <Select
              options={options()}
              value={""}
              label={"Select an option"}
              size={Size::Large}
              errors={vec!["Invalid selection".to_string()]}
            />
          </PreviewContainer>
        </div>
      </div>
      <PropTable props={props}/>
    </div>
  }
}
use yew::{function_component, html, Html};
use crate::components::Input;
use crate::previews::PreviewContainer;
use crate::types::Size;

#[function_component(InputPreview)]
pub(crate) fn input_preview() -> Html {
  let mut errors: Vec<String> = Vec::new();
  errors.push("Invalid input".to_string());

  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Input"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display input fields with various sizes and states."}
        </div>
        <pre class="code-block">
          <code>
{"yewi add input"}
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
              <Input placeholder={"Type here..."} input_size={Size::Small}/>
              <Input placeholder={"Type here..."} input_size={Size::Medium}/>
              <Input placeholder={"Type here..."} input_size={Size::Large}/>
            "#}
          >
            <Input placeholder={"Type here..."} input_size={Size::Small}/>
            <Input placeholder={"Type here..."} input_size={Size::Medium}/>
            <Input placeholder={"Type here..."} input_size={Size::Large}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Disabled"}
            code={r#"
              <Input placeholder={"Cannot type here..."} input_size={Size::Small} disabled=true/>
              <Input placeholder={"Cannot type here..."} input_size={Size::Medium} disabled=true/>
              <Input placeholder={"Cannot type here..."} input_size={Size::Large} disabled=true/>
            "#}
          >
            <Input placeholder={"Cannot type here..."} input_size={Size::Small} disabled=true/>
            <Input placeholder={"Cannot type here..."} input_size={Size::Medium} disabled=true/>
            <Input placeholder={"Cannot type here..."} input_size={Size::Large} disabled=true/>
          </PreviewContainer>
          <PreviewContainer
            title={"With error"}
            code={r#"
              <Input placeholder={"Type here..."} input_size={Size::Small} errors={errors.clone()}/>
              <Input placeholder={"Type here..."} input_size={Size::Medium} errors={errors.clone()}/>
              <Input placeholder={"Type here..."} input_size={Size::Large} errors={errors.clone()}/>
            "#}
          >
            <Input placeholder={"Type here..."} input_size={Size::Small} errors={errors.clone()}/>
            <Input placeholder={"Type here..."} input_size={Size::Medium} errors={errors.clone()}/>
            <Input placeholder={"Type here..."} input_size={Size::Large} errors={errors.clone()}/>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
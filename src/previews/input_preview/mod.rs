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
  }
}
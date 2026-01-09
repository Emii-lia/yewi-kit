use yew::{function_component, html, Html};
use crate::components::PasswordInput;
use crate::previews::PreviewContainer;
use crate::types::Size;

#[function_component(PasswordInputPreview)]
pub(crate) fn password_input_preview() -> Html {
  let mut errors: Vec<String> = Vec::new();
  errors.push("Invalid input".to_string());
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Password Input"}</h1>
      <div class="preview-list">
        <PreviewContainer
          title={"Default"}
          code={r#"
            <PasswordInput input_size={Size::Small}/>
            <PasswordInput/>
            <PasswordInput input_size={Size::Large}/>
          "#}
        >
          <PasswordInput input_size={Size::Small}/>
          <PasswordInput/>
          <PasswordInput input_size={Size::Large}/>
        </PreviewContainer>
        <PreviewContainer
          title={"Disabled"}
          code={r#"
            <PasswordInput disabled=true input_size={Size::Small}/>
            <PasswordInput disabled=true/>
            <PasswordInput disabled=true input_size={Size::Large}/>
          "#}
        >
          <PasswordInput disabled=true input_size={Size::Small}/>
          <PasswordInput disabled=true/>
          <PasswordInput disabled=true input_size={Size::Large}/>
        </PreviewContainer>
        <PreviewContainer
          title={"With errors"}
          code={r#"
            <PasswordInput errors={errors.clone()} input_size={Size::Small}/>
            <PasswordInput errors={errors.clone()}/>
            <PasswordInput errors={errors.clone()} input_size={Size::Large}/>
          "#}
        >
          <PasswordInput errors={errors.clone()} input_size={Size::Small}/>
          <PasswordInput errors={errors.clone()}/>
          <PasswordInput errors={errors.clone()} input_size={Size::Large}/>
        </PreviewContainer>
      </div>
    </div>
  }
}
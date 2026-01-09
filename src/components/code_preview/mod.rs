mod hooks;

use yew::{function_component, html, AttrValue, Html, Properties};
use crate::components::Button;
use crate::components::code_preview::hooks::{use_code_preview, HookParams, HookResponse};
use crate::types::{ButtonVariant, Size};

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
  pub code: AttrValue
}
#[function_component(CodePreview)]
pub(crate) fn code_preview(props: &Props) -> Html {
  let HookResponse {
    highlight_code,
    copied,
    on_copy
  } = use_code_preview(HookParams {
    code: props.code.clone(),
  });
  let code = highlight_code(&props.code);
  html! {
    <div class="CodePreview">
      <div class="CodePreview__header">
        <Button variant={ButtonVariant::Secondary} size={Size::Small} onclick={on_copy.clone()} class="CodePreview__copy-button">
          { if copied { "Copied!" } else { "Copy" } }
        </Button>
      </div>
      <pre class="CodePreview__code">
        <code>
          { code }
        </code>
      </pre>
    </div>
  }
}
mod hooks;

use yew::{function_component, html, AttrValue, Html, Properties};
use yew_icons::IconData;
use crate::components::{Button, ButtonVariant};
use crate::components::code_preview::hooks::{use_code_preview, HookParams, HookResponse};
use crate::types::Size;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
  pub code: AttrValue
}
#[function_component(CodePreview)]
pub(crate) fn code_preview(props: &Props) -> Html {
  let HookResponse {
    copied,
    on_copy
  } = use_code_preview(HookParams {
    code: props.code.clone(),
  });
  html! {
    <div class="CodePreview">
      <pre class="CodePreview__code">
        <code class="code-content">{&props.code}</code>
      </pre>
      <Button
        variant={ButtonVariant::Secondary}
        title={"Copy to clipboard"}
        icon={if copied { IconData::LUCIDE_CHECK } else { IconData::LUCIDE_COPY }}
        onclick={on_copy}
        size={Size::Small}
        class="CodePreview__copy-button"
      />
    </div>
  }
}
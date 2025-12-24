use yew::{function_component, html, Callback, Html};
use crate::components::{use_modal_store, Button, ModalHookResponse, OpenParams};
use crate::previews::PreviewContainer;
use crate::types::Size;

#[function_component(ModalPreview)]
pub(crate) fn modal_preview() -> Html {
  let ModalHookResponse {
    on_open,
    ..
  } = use_modal_store();

  let children: Html = html! {
    <div class="ex-modal-content">
      <h2 class="ex-modal-title">{ "This is a Modal" }</h2>
      <p class="ex-modal-description">{ "Modals are used to display content in a layer above the main application." }</p>
    </div>
  };

  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Modal"}</h1>
      <div class="preview-list">
        <PreviewContainer title={"Example"}>
          <Button onclick={Callback::from(move |_| {
              on_open.clone().emit(OpenParams {
                children: children.clone(),
                class: Some("".into()),
              });
            })}
          >
            {"Open Modal"}
          </Button>
        </PreviewContainer>
      </div>
    </div>
  }
}
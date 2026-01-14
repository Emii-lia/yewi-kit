use yew::{function_component, html, Callback, Html};
use crate::components::{use_modal_store, Button, ModalHookResponse, OpenParams};
use crate::previews::PreviewContainer;

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
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display floating content in a layer above the main application."}
        </div>
        <pre class="code-block">
          <code>
    {"yewi add button modal"}
          </code>
        </pre>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Example"}
            code={r#"
    // Callback
    let ModalHookResponse {
      on_open,
      ..
    } = use_modal_store();

    <Button onclick={Callback::from(move |_| {
        on_open.clone().emit(OpenParams {
          children: children.clone(),
          class: Some("".into()),
        });
      })}
    >
      {"Open Modal"}
    </Button>

    // Modal Provider on src/app/mod.rs
    <ModalProvider>
      <div class="app">
        <Modal/>
        <Switch<AppRoute> render={switch}/>
      </div>
    </ModalProvider>
            "#}
          >
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
    </div>
  }
}
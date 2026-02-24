use yew::{function_component, html, Callback, Html};
use crate::components::{use_modal_store, Button, CodePreview, ModalHookResponse, OpenParams};
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
        <CodePreview code={"yewi add modal button"}/>
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
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Usage"}
        </h2>
        <div class="preview-usages">
          <div class="preview-usage">
            <p class="preview-text">
              {"To use the Modal component, wrap your application in a"}
              <span class="mono__highlight">{" ModalProvider "}</span>
              {"and include the"}
              <span class="mono__highlight">{" Modal "}</span>
              {"component at the root level of your app."}
            </p>
            <CodePreview code={r#"
  #[function_component(App)]
  pub fn app() -> Html {
    html! {
      <BrowserRouter>
        <ModalProvider>
          <div class="app">
            <Modal/>
            <Switch<AppRoute> render={switch_main}/>
          </div>
        </ModalProvider>
      </BrowserRouter>
    }
  }
            "#} />
          </div>
          <div class="preview-usage">
            <p class="preview-text">
              {"To open a modal, use the"}
              <span class="mono__highlight">{" on_open "}</span>
              {"callback from the"}
              <span class="mono__highlight">{" use_modal_store "}</span>
              {"hook."}
            </p>
            <CodePreview code={r#"
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
            "#}/>
          </div>
          <div class="preview-usage">
            <p class="preview-text">
              <span class="mono__highlight">{" use_modal_store "}</span>
              {"hook also returns the"}
              <span class="mono__highlight">{" on_close "}</span>
              {"callback, which can be used to close the modal."}
            </p>
            <CodePreview code={r#"
  let ModalHookResponse {
    on_close,
    ..
  } = use_modal_store();
  <Button onclick={Callback::from(move |_| {
      on_close.clone().emit(());
    })}
    icon={IconData::LUCIDE_X}
  />
            "#}/>
          </div>
        </div>
      </div>
    </div>
  }
}
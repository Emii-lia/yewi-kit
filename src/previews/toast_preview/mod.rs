use yew::{function_component, html, Callback, Children, Html};
use crate::components::{use_toast_store, Button, ButtonVariant, CodePreview, ToastPosition, ToastState};
use crate::previews::PreviewContainer;

#[function_component(ToastPreview)]
pub(crate) fn toast_preview() -> Html {
  let toast = use_toast_store();

  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">{"Toast"}</h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display pop up messages to notify users of events."}
        </div>
        <CodePreview code={"yewi add toast"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer title={"Default"} code={r#"
    // Call
    let toast: ToastContext = use_toast();

    // Usage
    <Button onclick={{
      let toast = toast.clone();
      Callback::from(move |_| toast.default.emit((
        Children::new(vec![html!{<p>{"Hello World"}</p>}]),
        None
      )))
    }}>
      {"Show Toast"}
    </Button>
          "#}>
            <Button onclick={{
              let toast = toast.clone();
              Callback::from(move |_| toast.default.emit((
                Children::new(vec![html!{<p>{"Hello World"}</p>}]),
                None
              )))
            }}>
              {"Show Toast"}
            </Button>
          </PreviewContainer>
          <PreviewContainer title={"Success"} code={r#"
    // Call
    let toast: ToastContext = use_toast();

    // Usage
    <Button variant={ButtonVariant::Success} onclick={{
      let toast = toast.clone();
      Callback::from(move |_| toast.success.emit((
        Children::new(vec![html!{<p>{"Hello Success"}</p>}]),
        None
      )))
    }}>
      {"Show Toast"}
    </Button>
          "#}>
            <Button variant={ButtonVariant::Success} onclick={{
              let toast = toast.clone();
              Callback::from(move |_| toast.success.emit((
                Children::new(vec![html!{<p>{"Hello Success"}</p>}]),
                None
              )))
            }}>
              {"Show Toast"}
            </Button>
          </PreviewContainer>
          <PreviewContainer title={"Default"} code={r#"
    // Call
    let toast: ToastContext = use_toast();

    // Usage
    <Button variant={ButtonVariant::Danger} onclick={{
      let toast = toast.clone();
      Callback::from(move |_| toast.error.emit((
        Children::new(vec![html!{<p>{"Hello Error"}</p>}]),
        None
      )))
    }}>
      {"Show Toast"}
    </Button>
          "#}>
            <Button variant={ButtonVariant::Danger} onclick={{
              let toast = toast.clone();
              Callback::from(move |_| toast.error.emit((
                Children::new(vec![html!{<p>{"Hello Error"}</p>}]),
                None
              )))
            }}>
              {"Show Toast"}
            </Button>
          </PreviewContainer>
          <PreviewContainer title={"Position"} code={r#"
    // Call
    let toast: ToastContext = use_toast();

    // Usage
    <Button onclick={{
      let toast = toast.clone();
      Callback::from(move |_| toast.default.emit((
        Children::new(vec![html!{<p>{"Hello World"}</p>}]),
        Some(ToastState {
          position: Some(ToastPosition::TopRight),
          ..Default::default()
        })
      )))
    }}>
      {"Show Toast"}
    </Button>
    <Button onclick={{
      let toast = toast.clone();
      Callback::from(move |_| toast.default.emit((
        Children::new(vec![html!{<p>{"Hello World"}</p>}]),
        Some(ToastState {
          position: Some(ToastPosition::TopCenter),
          ..Default::default()
        })
      )))
    }}>
      {"Show Toast"}
    </Button>
          "#}>
            <Button onclick={{
              let toast = toast.clone();
              Callback::from(move |_| toast.default.emit((
                Children::new(vec![html!{<p>{"Hello World"}</p>}]),
                Some(ToastState {
                  position: Some(ToastPosition::TopRight),
                  ..Default::default()
                })
              )))
            }}>
              {"Show Top Right"}
            </Button>
            <Button onclick={{
              let toast = toast.clone();
              Callback::from(move |_| toast.default.emit((
                Children::new(vec![html!{<p>{"Hello World"}</p>}]),
                Some(ToastState {
                  position: Some(ToastPosition::TopCenter),
                  ..Default::default()
                })
              )))
            }}>
              {"Show Top Center"}
            </Button>
          </PreviewContainer>
        </div>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">{"Usage"}</h2>
        <div class="preview-usages">
          <div class="preview-usage">
            <p class="preview-text">
              {"Wrap your application in a "}
              <code class="mono__highlight">{"ToastProvider"}</code>
              {" and place "}
              <code class="mono__highlight">{"ToastContainer"}</code>
              {" at the root level."}
            </p>
            <CodePreview code={r#"
      #[function_component(App)]
      pub fn app() -> Html {
        html! {
          <BrowserRouter>
            <ToastProvider>
              <div class="app">
                <ToastContainer/>
                <Switch<AppRoute> render={switch_main}/>
              </div>
            </ToastProvider>
          </BrowserRouter>
        }
      }"#}/>
          </div>
          <div class="preview-usage">
            <p class="preview-text">
              {"Access the toast context via the "}
              <code class="mono__highlight">{"use_toast_store"}</code>
              {" hook. It exposes three callbacks:"}
            </p>
            <ul class="preview-usage-list">
              <li><code class="mono__highlight">{"default"}</code>{": renders a neutral toast"}</li>
              <li><code class="mono__highlight">{"success"}</code>{": renders a success toast"}</li>
              <li><code class="mono__highlight">{"error"}</code>{": renders an error toast"}</li>
            </ul>
            <p class="preview-text">
              {"Each callback accepts "}
              <code class="mono__highlight">{"(children: Children, options: Option<ToastState>)"}</code>
              {". Use "}
              <code class="mono__highlight">{"children"}</code>
              {" to set the toast content, and "}
              <code class="mono__highlight">{"options"}</code>
              {" to configure position, duration, and class."}
            </p>
            <CodePreview code={r#"
      #[function_component(MyComponent)]
      pub fn my_component() -> Html {
        let toast = use_toast_store();

        let on_click = {
          let toast = toast.clone();
          Callback::from(move |_| {
            toast.success((html! { <span>{"Saved!"}</span> }, None));
          })
        };

        html! {
          <button onclick={on_click}>{"Save"}</button>
        }
      }"#}/>
          </div>

        </div>
      </div>
    </div>
  }
}
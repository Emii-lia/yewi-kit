use yew::{function_component, html, Callback, Children, Classes, Html};
use crate::components::{use_toast_store, Button, ButtonVariant, ToastPosition, ToastState};
use crate::previews::PreviewContainer;

#[function_component(ToastPreview)]
pub(crate) fn toast_preview() -> Html {
  let toast = use_toast_store();

  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Toast"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display pop up messages to notify users of events."}
        </div>
        <pre class="code-block">
          <code>
    {"yewi add toast"}
          </code>
        </pre>
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
    </div>
  }
}
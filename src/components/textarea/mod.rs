use web_sys::Event;
use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub id: AttrValue,
  #[prop_or_default]
  pub label: AttrValue,
  #[prop_or_default]
  pub value: AttrValue,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or_default]
  pub placeholder: AttrValue,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub rows: AttrValue,
  #[prop_or_default]
  pub cols: AttrValue,
  #[prop_or_default]
  pub name: AttrValue,
  #[prop_or(true)]
  pub resize: bool,
  #[prop_or_default]
  pub max_length: Option<AttrValue>,
  #[prop_or_default]
  pub errors: Vec<String>,
  #[prop_or_default]
  pub on_change: Callback<Event>
}

#[function_component(Textarea)]
pub(crate) fn textarea(props: &Props) -> Html {
  html! {
    <div class="Textarea">
      <label
        class="textarea-container"
        for={&props.id}
      >
        {html! {
          if !props.label.clone().is_empty() {
            <span class="textarea-label">{props.label.clone()}</span>
          }
        }}
        <textarea
          id={&props.id}
          name={&props.name}
          rows={&props.rows}
          cols={&props.cols}
          placeholder={&props.placeholder}
          class={classes!(
            "textarea",
            &props.class,
            props.resize.then_some("resizable")
          )}
          value={props.value.clone()}
          onchange={props.on_change.clone()}
          disabled={props.disabled}
          maxlength={&props.max_length}
        />
      </label>
      {html! {
        if !props.errors.is_empty() {
          <div class="textarea-errors">
            {for props.errors.iter().map(|error|
              html! {
                <p class="textarea-error" key={error.to_string()}>{format!("{}. ", error)}</p>
              })
            }
          </div>
        }
      }}
    </div>
  }
}
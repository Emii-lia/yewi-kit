use yew::{classes, function_component, html, Html};
use crate::components::input::types::InputProps;
use crate::components::input::use_input::{use_input, HookParams, HookResponse};

mod types;
mod use_input;

#[function_component]
pub(crate) fn Input(props: &InputProps) -> Html {
  let HookResponse { size } = use_input(HookParams { size: props.input_size.clone() });
  html! {
    <div class="Input">
      <label
        r#for={props.id.clone()}
        class={classes!(
          "input-container",
          size.clone(),
          if props.disabled { "disabled" } else { "" }
        )}
      >
        <input
          ref={props.node_ref.clone()}
          class={classes!(
            "input-form",
            props.class.clone()
          )}

          id={props.id.clone()}
          name={props.name.clone()}
          type={props.r#type.clone()}
          value={props.value.clone()}
          placeholder={props.placeholder.clone()}

          disabled={props.disabled}
          readonly={props.readonly}
          required={props.required}
          autofocus={props.autofocus}
          autocomplete={props.autocomplete.clone()}

          min={props.min.clone()}
          max={props.max.clone()}
          step={props.step.clone()}
          pattern={props.pattern.clone()}
          maxLength={props.maxlength.clone()}
          minlength={props.minlength.clone()}
          tabindex={props.tabindex.clone()}
          oninput={props.oninput.clone()}
          onchange={props.onchange.clone()}
          onfocus={props.onfocus.clone()}
          onblur={props.onblur.clone()}
          onkeydown={props.onkeydown.clone()}
        />
      </label>
      {html! {
        if !props.errors.is_empty() {
          <div class="input-errors">
            {for props.errors.iter().map(|error|
              html! {
                <p class="input-error" key={error.clone()}>{format!("{}. ", error)}</p>
              })
            }
          </div>
        }
      }}
    </div>
  }
}
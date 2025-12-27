use yew::{classes, function_component, html, Html};
use crate::components::input::types::InputProps;
use crate::components::input::hooks::{use_input, HookParams, HookResponse};

mod types;
mod hooks;

#[function_component(Input)]
pub(crate) fn input(props: &InputProps) -> Html {
  let HookResponse { size } = use_input(HookParams { size: props.input_size.clone() });
  let errors = &props.errors;
  html! {
    <div class="Input">
      <label
        for={&props.id}
        class={classes!(
          "input-container",
          size,
          props.disabled.then_some("disabled")
        )}
      >
        <input
          ref={&props.node_ref}
          class={classes!(
            "input-form",
            &props.class
          )}

          id={&props.id}
          name={&props.name}
          type={&props.r#type}
          value={&props.value}
          placeholder={&props.placeholder}

          disabled={props.disabled}
          readonly={props.readonly}
          required={props.required}
          autofocus={props.autofocus}
          autocomplete={&props.autocomplete}

          min={&props.min}
          max={&props.max}
          step={&props.step}
          pattern={&props.pattern}
          maxLength={&props.maxlength}
          minlength={&props.minlength}
          tabindex={&props.tabindex}
          oninput={props.oninput.clone()}
          onchange={props.onchange.clone()}
          onfocus={props.onfocus.clone()}
          onblur={props.onblur.clone()}
          onkeydown={props.onkeydown.clone()}
        />
      </label>
      {html! {
        if !errors.is_empty() {
          <div class="input-errors">
            {for errors.iter().map(|error|
              html! {
                <p class="input-error" key={error.to_string()}>{format!("{}. ", error)}</p>
              })
            }
          </div>
        }
      }}
    </div>
  }
}
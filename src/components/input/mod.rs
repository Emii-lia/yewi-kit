use yew::{classes, function_component, html, Html};
use yew_icons::Icon;
use crate::components::input::types::InputProps;
use crate::components::input::hooks::{use_input, HookParams, HookResponse};
use crate::types::Size;

mod types;
mod hooks;

#[function_component(Input)]
pub(crate) fn input(props: &InputProps) -> Html {
  let HookResponse { size } = use_input(HookParams { size: props.input_size.clone() });
  let errors = &props.errors;
  let icon = props.icon;
  let icon_size = match props.input_size {
    Size::Small => "1rem",
    Size::Medium => "1.25rem",
    Size::Large => "1.5rem",
  };

  html! {
    <div class="Input">
      <label
        for={&props.id}
        class={classes!(
          "input-container",
          size,
          icon.is_some().then_some("with-icon"),
          props.disabled.then_some("disabled")
        )}
      >
        {html! {
          if let Some(icon) = icon {
            <Icon data={icon} width={icon_size.to_string()} height={icon_size.to_string()} class={"input-icon"} />
          }
        }}
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
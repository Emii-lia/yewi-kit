mod types;
mod hooks;

use yew::{classes, function_component, html, Callback, Html};
use crate::components::Button;
use crate::components::password_input::hooks::{use_password_input, HookResponse};
use crate::components::password_input::types::PasswordInputProps;
use crate::types::{ButtonVariant, Size};

#[function_component(PasswordInput)]
pub(crate) fn password_input(props: &PasswordInputProps) -> Html {
  let size_class = format!("{:?}", props.input_size).to_lowercase();
  let errors = &props.errors;
  let HookResponse {
    is_visible,
    toggle_visible
  } = use_password_input();

  let eye_on = html! {
    <svg width="16" height="16" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M10 4C5.522 4 1.731 6.943 0.457 11C1.731 15.057 5.522 18 10 18C14.478 18 18.268 15.057 19.543 11C18.268 6.943 14.478 4 10 4ZM10 16C6.326 16 3.122 13.627 1.929 10C3.122 6.373 6.326 4 10 4C13.674 4 16.878 6.373 18.071 10C16.878 13.627 13.674 16 10 16ZM10 7C8.343 7 7 8.343 7 10C7 11.657 8.343 13 10 13C11.657 13 13 11.657 13 10C13 8.343 11.657 7 10 7ZM10 11C9.447 11 9 10.553 9 10C9 9.447 9.447 9 10 9C10.553 9 11 9.447 11 10C11 10.553 10.553 11 10 11Z" fill="#999999"/>
    </svg>
  };
  let eye_off = html! {
    <svg width="16" height="16" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M3.70741 2.29304C3.5188 2.11088 3.2662 2.01009 3.00401 2.01237C2.74181 2.01465 2.491 2.11981 2.30559 2.30522C2.12018 2.49063 2.01501 2.74144 2.01273 3.00364C2.01045 3.26584 2.11125 3.51844 2.29341 3.70704L16.2934 17.707C16.482 17.8892 16.7346 17.99 16.9968 17.9877C17.259 17.9854 17.5098 17.8803 17.6952 17.6949C17.8806 17.5095 17.9858 17.2586 17.9881 16.9964C17.9904 16.7342 17.8896 16.4816 17.7074 16.293L16.2344 14.82C17.7919 13.5782 18.9437 11.9 19.5424 10C18.2684 5.94304 14.4784 3.00004 10.0004 3.00004C8.43284 2.99792 6.88691 3.3659 5.48841 4.07404L3.70741 2.29304ZM7.96841 6.55304L9.48241 8.06804C9.8215 7.97799 10.1783 7.97859 10.5171 8.06977C10.8559 8.16095 11.1648 8.33952 11.4128 8.58761C11.6609 8.83569 11.8395 9.14458 11.9307 9.48337C12.0219 9.82216 12.0225 10.179 11.9324 10.518L13.4464 12.032C13.8974 11.2681 14.0816 10.3758 13.9701 9.49572C13.8585 8.6156 13.4575 7.79754 12.8302 7.17022C12.2029 6.5429 11.3848 6.14193 10.5047 6.03039C9.62461 5.91885 8.73239 6.10206 7.96841 6.55304Z" fill="#999999"/>
      <path d="M12.454 16.697L9.75001 13.992C8.77769 13.9311 7.86104 13.5174 7.17207 12.8286C6.4831 12.1398 6.06918 11.2233 6.00801 10.251L2.33501 6.57797C1.49022 7.58399 0.852357 8.74689 0.458008 9.99997C1.73201 14.057 5.52301 17 10 17C10.847 17 11.669 16.895 12.454 16.697Z" fill="#999999"/>
    </svg>
  };

  html! {
    <div class="PasswordInput">
      <label
          for={&props.id}
          class={classes!(
            "PasswordInput__label",
            size_class,
            props.disabled.then_some("disabled")
          )}
      >
        <input
          ref={&props.node_ref}
          class={classes!(
            "PasswordInput__input",
            &props.class
          )}

          id={&props.id}
          type={if is_visible { "text" } else { "password" } }
          name={&props.name}
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
        <Button
          variant={ButtonVariant::Tertiary}
          size={Size::Small}
          class="eye-toggle"
          onclick={Callback::from(move |_|{
            toggle_visible.emit(());
          })}
        >
          {if is_visible { eye_on } else { eye_off }}
        </Button>
      </label>
      {html! {
        if !errors.is_empty() {
          <div class="PasswordInput__errors">
            {for errors.iter().map(|error|
              html! {
                <p class="PasswordInput__error" key={error.to_string()}>{format!("{}. ", error)}</p>
              })
            }
          </div>
        }
      }}
    </div>
  }
}
mod types;

use web_sys::{DragEvent, Event, File, FileList, HtmlInputElement};
use yew::{classes, function_component, html, use_node_ref, AttrValue, Callback, Classes, Html, Properties, TargetCast};
use crate::components::Button;
use crate::types::{ButtonVariant, Size};
pub use types::FileInputType;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub multiple: bool,
  #[prop_or_default]
  pub accept: AttrValue,
  #[prop_or_default]
  pub onchange: Callback<FileList>,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or_default]
  pub value: Option<FileList>,
  #[prop_or(FileInputType::Input)]
  pub r#type: FileInputType,
  #[prop_or(ButtonVariant::Primary)]
  pub button_variant: ButtonVariant,
  #[prop_or_default]
  pub children: Html,
  #[prop_or(Size::Medium)]
  pub button_size: Size,
}

#[function_component(FileInput)]
pub(crate) fn file_input(props: &Props) -> Html {
  let file_input_ref = use_node_ref();
  html! {
    <div class="FileInput">
      {
        match props.r#type {
          FileInputType::Input => html! {
            <input
              class={classes!("file-input-element", &props.class)}
              type="file"
              multiple={props.multiple}
              accept={&props.accept}
              disabled={props.disabled}
              onchange={
                let onchange = props.onchange.clone();
                Callback::from(move |e: Event| {
                  let input: HtmlInputElement = e.target_unchecked_into();
                  if let Some(files) = input.files() {
                    onchange.emit(files);
                  }
                })
              }
            />
          },
          FileInputType::DnD => html! {
            <label
              class={classes!(
                "file-dropzone",
                props.disabled.then_some("disabled")
              )}
              ondragover={
                Callback::from(|e: web_sys::DragEvent| {
                  e.prevent_default();
                })
              }
              ondrop={
                let onchange = props.onchange.clone();
                Callback::from(move |e: DragEvent| {
                  e.prevent_default();
                  if let Some(files) = e.data_transfer().and_then(|dt| dt.files()) {
                    onchange.emit(files);
                  }
                })
              }
              for="file-input-dnd"
            >
              { "Drag and drop files here or click to select files." }
              <input
                type="file"
                multiple={props.multiple}
                accept={&props.accept}
                class="file-input-hidden"
                id="file-input-dnd"
                name="file-input-dnd"
                disabled={props.disabled}
              />
            </label>
          },
          FileInputType::Button => html! {
            <label
              class="file-input-button"
              for="file-input-button"
            >
              <Button
                variant={props.button_variant.clone()}
                size={props.button_size.clone()}
                disabled={props.disabled}
                onclick={
                  let file_input_ref = file_input_ref.clone();
                  Callback::from(move |_| {
                    if let Some(file_input) = file_input_ref.cast::<HtmlInputElement>() {
                      file_input.click();
                    }
                  })
                }
              >
                { props.children.clone()}
              </Button>
              <input
                type="file"
                multiple={props.multiple}
                accept={&props.accept}
                class="file-input-hidden"
                id="file-input-button"
                name="file-input-button"
                ref={file_input_ref}
                disabled={props.disabled}
                onchange={
                  let onchange = props.onchange.clone();
                  Callback::from(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    if let Some(files) = input.files() {
                      onchange.emit(files);
                    }
                  })
                }
              />
            </label>
          }
        }
      }
    </div>
  }
}
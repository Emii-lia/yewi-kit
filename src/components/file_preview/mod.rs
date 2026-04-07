pub mod types;

use web_sys::{MouseEvent};
use yew::{classes, component, html, Callback, Classes, Html, Properties};
use yew_icons::{Icon, IconData};
use crate::components::button::{Button, ButtonVariant};
use crate::components::file_preview::types::FileValue;
use crate::types::Size;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub file: Option<FileValue>,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub onclick: Callback<()>,
  #[prop_or_default]
  pub onremove: Callback<()>,
}

#[component(FilePreview)]
pub fn file_preview(props: &Props) -> Html {
  let Props { file, class, onclick, onremove } = props;
  html! {
    <div
      class={classes!("FilePreview", class)}
      onclick={{
        let onclick = onclick.clone();
        Callback::from(move |e: MouseEvent| {
          e.stop_propagation();
          onclick.emit(());
        })
      }}
    >
      {if let Some(file) = file {
        match file {
          FileValue::File(file) => {
            if file.type_().starts_with("image/") {
              let url = web_sys::Url::create_object_url_with_blob(&file).unwrap_or_default();
              html! {
                <div class="file-preview-content">
                  <div
                    class="file-preview-image"
                    title={file.name()}
                    style={format!("--image: url('{}')", url)}
                  />
                  <Button
                    class="preview-close-btn"
                    size={Size::Small}
                    icon={IconData::LUCIDE_X}
                    variant={ButtonVariant::Tertiary}
                    rounded=true
                    onclick={{
                      let onremove = onremove.clone();
                      Callback::from(move |e: MouseEvent| {
                        e.stop_propagation();
                        onremove.emit(());
                      })
                    }}
                  />
                </div>
              }
            } else if file.type_().starts_with("video/") || file.type_().starts_with("audio/") {
              html! {
                <div class="file-preview-content">
                  <video controls=true class="file-preview-video">
                    <source src={web_sys::Url::create_object_url_with_blob(&file).unwrap_or_default()} type={file.type_()}/>
                    { "Your browser does not support the video tag." }
                  </video>
                  <Button
                    class="preview-close-btn"
                    size={Size::Small}
                    icon={IconData::LUCIDE_X}
                    variant={ButtonVariant::Tertiary}
                    rounded=true
                    onclick={{
                      let onremove = onremove.clone();
                      Callback::from(move |e: MouseEvent| {
                        e.stop_propagation();
                        onremove.emit(());
                      })
                    }}
                  />
                </div>
              }
            } else {
              html! {
                <div class="file-preview-content">
                  <div class="file-preview-other">
                    <Icon data={IconData::LUCIDE_FILE} class="file-preview-icon"/>
                    <p class="file-preview-name">{ file.name() }</p>
                  </div>
                  <Button
                    class="preview-close-btn"
                    size={Size::Small}
                    icon={IconData::LUCIDE_X}
                    variant={ButtonVariant::Tertiary}
                    rounded=true
                    onclick={{
                      let onremove = onremove.clone();
                      Callback::from(move |e: MouseEvent| {
                        e.stop_propagation();
                        onremove.emit(());
                      })
                    }}
                  />
                </div>
              }
            }
          },
          FileValue::Url(url, r#type, name) => html! {
            <div class="file-preview-content">
              {
                if let Some(r#type) = r#type {
                  if r#type.starts_with("image/") {
                    html! {
                      <img src={url.clone()} alt={name.clone().unwrap_or_default()} class="file-preview-image"/>
                    }
                  } else if r#type.starts_with("video/") || r#type.starts_with("audio/") {
                    html! {
                      <video controls=true class="file-preview-video">
                        <source src={url.clone()} type={r#type.clone()}/>
                        { "Your browser does not support the video tag." }
                      </video>
                    }
                  } else {
                    html! {
                      <div class="file-preview-other">
                        <Icon data={IconData::LUCIDE_FILE} class="file-preview-icon"/>
                        <p class="file-preview-name">{ name.clone().unwrap_or_default() }</p>
                      </div>
                    }
                  }
                } else {
                  html! {
                    <p class="file-preview-name">{ name.clone().unwrap_or_default() }</p>
                  }
                }
              }
              <Button
                class="preview-close-btn"
                size={Size::Small}
                icon={IconData::LUCIDE_X}
                variant={ButtonVariant::Tertiary}
                rounded=true
                onclick={{
                  let onremove = onremove.clone();
                  Callback::from(move |e: MouseEvent| {
                    e.stop_propagation();
                    onremove.emit(());
                  })
                }}
              />
            </div>
          }
        }
      }
      else {
        html! {
          <div class="file-preview-content">
            <p class="file-preview-placeholder">{ "No file selected" }</p>
          </div>
        }
      }}
    </div>
  }
}

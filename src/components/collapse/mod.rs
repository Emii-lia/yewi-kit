mod types;
mod hooks;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};
pub(crate) use types::*;
use crate::components::collapse::hooks::{use_collapse, HookResponse};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub summary: AttrValue,
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(CollapseVariant::Focus)]
  pub variant: CollapseVariant,
}
#[function_component(Collapse)]
pub(crate) fn collapse(props: &Props) -> Html {
  let HookResponse { is_open, toggle,checkbox_ref } = use_collapse();

  match props.variant {
    CollapseVariant::Toggle => {
      html! {
        <div
          class={classes!("Collapse__toggle", is_open.then_some("open"))}
          onclick={{
            let toggle = toggle.clone();
            Callback::from(move |_e| {
              toggle.emit(());
            })
          }}
        >
          <input
            type={"checkbox"}
            class={"collapse-state"}
            onchange={{
              let toggle = toggle.clone();
              Callback::from(move |_| {
                toggle.emit(());
              })
            }}
            checked={is_open}
            ref={checkbox_ref.clone()}
          />
          <div class="collapse-trigger">{&props.summary}</div>
          <div class={classes!("collapse-content", &props.class)}>
            {props.children.clone()}
          </div>
        </div>
      }
    },
    CollapseVariant::Focus => {
      html! {
        <div class="Collapse" tabindex={"1"}>
          <div class="collapse-trigger">{&props.summary}</div>
          <div class={classes!("collapse-content", &props.class)}>
            {props.children.clone()}
          </div>
        </div>
      }
    }
  }
}
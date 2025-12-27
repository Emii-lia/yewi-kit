mod hooks;

use yew::{classes, function_component, html, Callback, Children, ChildrenWithProps, Classes, Html, Properties};
use crate::components::tabs::hooks::{use_tabs, HookResponse};
use crate::types::Color;

#[derive(Properties, PartialEq, Clone)]
pub struct TabProps {
  pub label: Html,
  pub value: String,
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub classes: Classes,
}

#[function_component(Tab)]
pub fn tab(_props: &TabProps) -> Html {
  Html::default()
}

#[derive(Properties, PartialEq, Clone)]
pub struct TabsProps {
  pub children: ChildrenWithProps<Tab>,
  #[prop_or_default]
  pub active_tab: Option<String>,
  #[prop_or_default]
  pub on_tab_change: Callback<String>,
  #[prop_or_default]
  pub classes: Classes,
  #[prop_or(Color::Transparent)]
  pub color: Color
}

#[function_component(Tabs)]
pub(crate) fn tabs(props: &TabsProps) -> Html {
  let HookResponse { active, tabs } = use_tabs(props);
  let color_class = format!("{:?}", props.color).to_lowercase();

  html! {
    <div class="Tabs">
      <div class="tab-list" role="tablist">
        {for tabs.iter().map(|tab| {
          let value = tab.props.value.clone();
          let is_active = *active == value;
          let onclick = {
            let active = active.clone();
            let on_tab_change = props.on_tab_change.clone();
            Callback::from(move |_| {
              active.set(value.clone());
              on_tab_change.emit(value.clone());
            })
          };

          html! {
            <button
              class={classes!(
                "Tab",
                is_active.then_some("is-active"),
                &color_class
              )}
              {onclick}
            >
              { tab.props.label.clone() }
            </button>
          }
        })}
      </div>
      {for tabs.iter().filter(|tab| *active == tab.props.value).map(|tab| {
        html! {
          <div class={classes!("tab-panel", &tab.props.classes)} role="tabpanel">
            { for tab.props.children.clone() }
          </div>
        }
      })}
    </div>
  }
}
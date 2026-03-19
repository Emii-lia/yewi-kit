pub mod types;

use yew::{classes, function_component, html, Classes, Html, NodeRef, Properties};
use yew_icons::{Icon, IconData};
use yew_router::{
  Routable,
  components::Link as RouterLink
};
use crate::components::link::types::LinkVariant;

#[derive(Properties, PartialEq ,Clone)]
pub struct Props<R>
where
  R: Routable
{
  pub href: R,
  #[prop_or_default]
  pub children: Html,
  #[prop_or(LinkVariant::Default)]
  pub variant: LinkVariant,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub anchor_ref: NodeRef,
  #[prop_or_default]
  pub icon: Option<IconData>
}

#[function_component(Link)]
pub fn link<R>(props: &Props<R>) -> Html
where
  R: Routable + 'static,
{

  html! {
    <RouterLink<R>
      to={props.href.clone()}
      classes={classes!(
        "Link",
        props.variant.as_str(),
        &props.class,
      )}
      anchor_ref={&props.anchor_ref}
    >
      {html! {
        if let Some(icon) = &props.icon {
          <Icon data={icon.clone()} class={classes!("link-icon")} />
        }
      }}
      { props.children.clone() }
    </RouterLink<R>>
  }
}
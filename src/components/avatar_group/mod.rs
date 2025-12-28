mod types;
mod hooks;

use std::rc::Rc;
use yew::{classes, function_component, html, ChildrenWithProps, Classes, Html, Properties};
use crate::components::Avatar;
use crate::components::avatar_group::hooks::{use_avatar_group, HookParams, HookResponse};
use crate::types::{Color, Size};
pub(crate) use types::AvatarGroupVariant;

#[derive(Clone, PartialEq, Properties)]
pub struct AvatarGroupProps {
  #[prop_or(Size::Medium)]
  pub size: Size,
  #[prop_or_default]
  pub rounded: bool,
  #[prop_or_default]
  pub with_border: bool,
  #[prop_or(Color::Blue)]
  pub color: Color,
  #[prop_or(4)]
  pub max: usize,
  #[prop_or(AvatarGroupVariant::Linear)]
  pub variant: AvatarGroupVariant,
  #[prop_or_default]
  pub class: Classes,
  pub children: ChildrenWithProps<Avatar>,
}
#[function_component(AvatarGroup)]
pub(crate) fn avatar_group(props: &AvatarGroupProps) -> Html {
  let variant_class = format!("{:?}", props.variant).to_lowercase();
  let HookResponse {
    visible_avatars,
    hidden_size,
    hidden_titles
  } = use_avatar_group(HookParams { max: props.max, children: props.children.clone() });
  
  html! {
    <div
      class={classes!(
        "AvatarGroup",
        variant_class,
        &props.class
      )}
    >
      {for visible_avatars.iter().enumerate().map(|(i, v)| {
        let base_props = Rc::unwrap_or_clone(v.props.clone());
        html! {
          <Avatar
            color={props.color.clone()}
            size={props.size.clone()}
            rounded={props.rounded}
            with_border={props.with_border}
            key={i}
            class="avatar-group-item"
            style={
              if props.variant == AvatarGroupVariant::Stacked {
                format!("margin-top: -{}rem; margin-left: {}rem;", i as f32 /1.5, i as f32 / 1.5)
              } else {
                "".to_string()
              }
            }
            ..base_props
          />
        }
      })}
      {html! {
        if hidden_size > 0 && props.variant == AvatarGroupVariant::Linear {
          <Avatar
            color={props.color.clone()}
            size={props.size.clone()}
            rounded={props.rounded}
            with_border={props.with_border}
            alt={format!("+{}", hidden_size)}
            title={hidden_titles.to_string()}
            class="avatar-group-item"
            no_split=true
          />
        }
      }}
    </div>
  }
}
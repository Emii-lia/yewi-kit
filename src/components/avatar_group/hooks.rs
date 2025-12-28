use yew::{hook, ChildrenWithProps, Properties};
use yew::virtual_dom::VChild;
use crate::components::{Avatar};

#[derive(Clone, PartialEq)]
pub struct HookResponse {
  pub visible_avatars: Vec<VChild<Avatar>>,
  pub hidden_titles: String,
  pub hidden_size: usize,
}

#[derive(Properties, Clone, PartialEq)]
pub struct HookParams {
  pub max: usize,
  pub children: ChildrenWithProps<Avatar>
}
#[hook]
pub(crate) fn use_avatar_group(params: HookParams) -> HookResponse {
  let avatars: Vec<VChild<Avatar>> = params.children
    .iter()
    .filter_map(|child| child.clone().try_into().ok())
    .collect();
  let (visible_avatars, hidden_avatars) = {
    let max = params.max;
    let (visible_avatars, hidden_avatars): ( Vec<_>,  Vec<_>) = avatars
      .into_iter()
      .enumerate()
      .partition(|(i, _)| *i  < max);
    (visible_avatars, hidden_avatars)
  };

  let hidden_titles = hidden_avatars.iter()
    .map(|(_, av)| av.props.alt.to_string())
    .collect::<Vec<String>>().join(", ");

  let hidden_size = hidden_avatars.len();

  HookResponse {
    visible_avatars: visible_avatars.into_iter().map(|(_, av)| av).collect(),
    hidden_titles,
    hidden_size
  }
}
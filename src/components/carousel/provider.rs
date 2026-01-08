use std::borrow::BorrowMut;
use std::collections::HashMap;
use yew::{function_component, html, use_memo, use_state, Callback, Children, ContextProvider, Html, Properties};
use crate::components::carousel::hooks::{use_carousel, HookParams};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CarouselContextType {
  pub(crate) current_index: usize,
  pub(crate) total_items: usize,
  pub(crate) go_to: Callback<usize>,
  pub(crate) next: Callback<()>,
  pub(crate) previous: Callback<()>,
  pub(crate) is_auto_playing: bool,
  pub(crate) toggle_auto_play: Callback<()>,
  pub(crate) register_item: Callback<String, usize>
}

#[derive(PartialEq, Properties, Clone)]
pub(crate) struct CarouselProviderProps {
  pub children: Children,
  pub auto_play: Option<bool>,
  pub auto_play_interval: Option<f64>,
}

#[function_component(CarouselProvider)]
pub(crate) fn carousel_provider(props: &CarouselProviderProps) -> Html {
  let item_count = use_state(|| 0);
  let item_map = use_state(|| HashMap::<String, usize>::new());

  let register_item = {
    let item_count = item_count.clone();
    let item_map = item_map.clone();
    Callback::from(move |id: String| {
      let mut map = (*item_map).clone();
      if !map.contains_key(&id) {
        let new_index = map.len() as usize;
        map.insert(id, new_index);
        item_count.set(map.len());
        item_map.set(map);
        new_index
      } else {
        *map.get(&id).unwrap_or(&0)
      }
    })
  };

  let carousel = use_carousel(HookParams {
    total_items: *item_count,
    auto_play_interval: props.auto_play_interval,
    auto_play: props.auto_play,
  });
  let value = use_memo(
    (carousel, *item_count, register_item),
    |(carousel, item_count, register_item)| {
      CarouselContextType {
        current_index: carousel.current_index,
        total_items: *item_count,
        go_to: carousel.go_to.clone(),
        next: carousel.next.clone(),
        previous: carousel.previous.clone(),
        is_auto_playing: carousel.is_auto_playing,
        toggle_auto_play: carousel.toggle_auto_play.clone(),
        register_item: register_item.clone(),
      }
    }
  );

  html! {
    <ContextProvider<CarouselContextType> context={(*value).clone()}>
      { props.children.clone() }
    </ContextProvider<CarouselContextType>>
  }
}
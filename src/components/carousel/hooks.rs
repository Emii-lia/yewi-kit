use gloo::console::log;
use gloo::timers::callback::Interval;
use yew::{hook, use_effect_with, use_state, Callback};

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct HookParams {
  pub total_items: usize,
  pub auto_play_interval: Option<f64>,
  pub auto_play: Option<bool>,
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct HookResponse {
  pub current_index: usize,
  pub go_to: Callback<usize>,
  pub next: Callback<()>,
  pub previous: Callback<()>,
  pub is_auto_playing: bool,
  pub toggle_auto_play: Callback<()>,
}
#[hook]
pub(crate) fn use_carousel(params: HookParams) -> HookResponse {
  let current_index = use_state(|| 0);
  let is_auto_playing = use_state(|| params.auto_play.unwrap_or(false));
  let total_items = params.total_items;
  let auto_play_interval = params.auto_play_interval;

  let go_to = {
    let current_index = current_index.clone();
    Callback::from(move |index: usize| {
      current_index.set(index % total_items);
    })
  };

  let next = {
    let current_index = current_index.clone();
    Callback::from(move |_| {
      current_index.set((*current_index + 1) % total_items);
    })
  };

  let previous = {
    let current_index = current_index.clone();
    Callback::from(move |_| {
      current_index.set(if *current_index == 0 {
        total_items - 1
      } else {
        *current_index - 1
      });
    })
  };

  let toggle_auto_play = {
    let is_auto_playing = is_auto_playing.clone();
    Callback::from(move |_| {
      is_auto_playing.set(!*is_auto_playing);
    })
  };
  {
    let is_auto_playing = is_auto_playing.clone();
    let current_index = current_index.clone();
    use_effect_with(
      (*is_auto_playing, auto_play_interval, total_items, current_index.clone()),
      move |(_is_auto_playing, _auto_play_interval, _total_items, _current_index)| {
        let mut interval_handle: Option<Interval> = None;
        if total_items != 0 && *is_auto_playing {
          let interval_ms = auto_play_interval.unwrap_or(3000.0) as u32;
          let current_index = current_index.clone();
          interval_handle = Some(Interval::new(interval_ms, move || {
            current_index.set((*current_index + 1) % total_items);
          }));
        }
        move || drop(interval_handle)
      },
    );
  }
  {
    let current_index = current_index.clone();
    use_effect_with(
      current_index.clone(),
      move |_| {
        log!(format!("Carousel current index changed to {}", *current_index));
      }
    );
  }

  HookResponse {
    current_index: *current_index,
    go_to,
    next,
    previous,
    is_auto_playing: *is_auto_playing,
    toggle_auto_play,
  }
}
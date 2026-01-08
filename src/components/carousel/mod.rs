use yew::{function_component, html, Callback, Children, Html, Properties};
use yew::html::ChildrenProps;
use crate::components::carousel::provider::CarouselProvider;
use crate::components::carousel::store::use_carousel_store;

mod hooks;
mod provider;
mod store;

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct CarouselItemProps {
  pub id: String,
  pub children: Children
}
#[function_component(CarouselItem)]
pub(crate) fn carousel_item(props: &CarouselItemProps) -> Html {
  let carousel = use_carousel_store();
  let index: usize = carousel.register_item.emit(props.id.clone());

  html! {
    <div
      class="CarouselItem"
      data-active={if carousel.current_index == index { "true" } else { "false" }}
    >
      { props.children.clone() }
    </div>
  }
}

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct CarouselContentProps {
  pub children: Children
}

#[function_component(CarouselContent)]
pub(crate) fn carousel_content(props: &CarouselContentProps) -> Html {
  html! {
    <div class="CarouselContent">
      { for props.children.iter() }
    </div>
  }
}

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct CarouselControlsProps {
  #[prop_or(true)]
  pub show_arrows: bool,
  #[prop_or(true)]
  pub show_dots: bool,
  #[prop_or(false)]
  pub show_auto_play_toggle: bool,
}

#[function_component(CarouselControls)]
pub(crate) fn carousel_controls(props: &CarouselControlsProps) -> Html {
  let carousel = use_carousel_store();
  html! {
    <>
      {
        html! {
          if props.show_arrows {
            <>
              <button
                onclick={Callback::from(move |_| {
                  carousel.previous.emit(());
                })}
                data-direction="prev"
                aria-label="Previous"
                class="CarouselControls__arrow-btn"
              >
                { "‹" }
              </button>
              <button
                onclick={Callback::from(move |_| {
                  carousel.next.emit(());
                })}
                data-direction="next"
                aria-label="Next"
                class="CarouselControls__arrow-btn"
              >
                { "›" }
              </button>
            </>
          }
        }
      }
      {
        html! {
          if props.show_dots {
            <div class="CarouselControls__dots">
              {
                (0..carousel.total_items).map(|index| {
                  let go_to = carousel.go_to.clone();
                  html! {
                    <button
                      class="CarouselControls__dot"
                      data-active={if carousel.current_index == index { "true" } else { "false" }}
                      onclick={Callback::from(move |_| go_to.emit(index))}
                      key={index}
                      aria-label={format!("Go to slide {}", index + 1)}
                    />
                  }
                }).collect::<Html>()
              }
            </div>
          }
        }
      }
      {
        html! {
          if props.show_auto_play_toggle {
            <button
              class="CarouselControls__autoplay"
              onclick={Callback::from(move |_| {
                carousel.toggle_auto_play.emit(());
              })}
              data-playing={if carousel.is_auto_playing { "true" } else { "false" }}
            >
              { if carousel.is_auto_playing { "Pause" } else { "Play" } }
            </button>
          }
        }
      }
    </>
  }
}

#[function_component(CarouselWrapper)]
pub(crate) fn carousel_wrapper(props: &ChildrenProps) -> Html {
  html! {
    <div class="CarouselWrapper">
      { props.children.clone() }
    </div>
  }
}

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct CarouselProps {
  pub children: Children,
  #[prop_or(false)]
  pub auto_play: bool,
  #[prop_or(3000.0)]
  pub auto_play_interval: f64,
}

#[function_component(Carousel)]
pub(crate) fn carousel(props: &CarouselProps) -> Html {
  html! {
    <CarouselProvider
      auto_play={Some(props.auto_play)}
      auto_play_interval={Some(props.auto_play_interval)}
    >
      <div class="Carousel">
        { props.children.clone() }
      </div>
    </CarouselProvider>
  }
}
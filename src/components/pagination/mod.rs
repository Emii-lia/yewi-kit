mod hooks;
mod types;

use yew::{classes, component, html, Callback, Classes, Html, Properties};
use yew_icons::IconData;
use crate::components::button::{Button, ButtonVariant};
pub use crate::components::pagination::types::PaginationVariant;
use crate::components::pagination::hooks::{use_pagination, PaginationHookParams};
use crate::types::Size;

#[derive(Properties, PartialEq ,Clone)]
pub struct Props {
  pub count: usize,
  #[prop_or(1)]
  pub current: usize,
  #[prop_or_default]
  pub on_change: Callback<usize>,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(PaginationVariant::Default)]
  pub variant: PaginationVariant,
  #[prop_or(false)]
  pub rounded: bool,
  #[prop_or(Size::Medium)]
  pub size: Size
}

#[component(Pagination)]
pub fn pagination(props: &Props) -> Html {
  let (
    current_page,
    go_to_page,
    page_numbers
  ) = use_pagination(PaginationHookParams {
    current: props.current,
    on_change: props.on_change.clone(),
    count: props.count
  });

  let btn_variant: ButtonVariant = match props.variant {
    PaginationVariant::Default => ButtonVariant::Tertiary,
    PaginationVariant::Outlined => ButtonVariant::Secondary
  };

  html! {
    <div class={classes!("Pagination", &props.class)}>
      <Button
        icon={IconData::LUCIDE_CHEVRON_LEFT}
        variant={btn_variant.clone()}
        size={props.size.clone()}
        rounded={props.rounded}
        disabled={current_page == 1}
        onclick={{
          let go_to_page = go_to_page.clone();
          Callback::from(move |_| {
            go_to_page.emit(current_page.clone() - 1);
          })}
        }
      />
      <div class="Pagination__numbers">
        {for page_numbers.iter().map(|number| {
          if number == "..." {
            html! { <span class="Pagination__ellipsis">{number}</span> }
          } else {
            let page_num = number.parse::<usize>().unwrap_or(1);
            html! {
              <Button
                key={number.clone()}
                variant={if page_num == current_page { ButtonVariant::Primary } else { btn_variant.clone() }}
                size={props.size.clone()}
                rounded={props.rounded}
                onclick={{
                  let go_to_page = go_to_page.clone();
                  Callback::from(move |_| {
                    go_to_page.emit(page_num);
                  })}
                }
              >
                {number}
              </Button>
            }
          }
        })}
      </div>
      <Button
        icon={IconData::LUCIDE_CHEVRON_RIGHT}
        variant={btn_variant.clone()}
        size={props.size.clone()}
        rounded={props.rounded}
        disabled={current_page == props.count}
        onclick={{
          let go_to_page = go_to_page.clone();
          Callback::from(move |_| {
            go_to_page.emit(current_page.clone() + 1);
          })}
        }
      />
    </div>
  }
}
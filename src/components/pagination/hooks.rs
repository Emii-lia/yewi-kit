use yew::{hook, use_state, Callback};

#[derive(Clone)]
pub(crate) struct PaginationHookParams {
  pub(crate) count: usize,
  pub(crate) current: usize,
  pub(crate) on_change: Callback<usize>
}

fn get_page_numbers(count: usize, current: usize) -> Vec<String> {
  let mut page_numbers = Vec::new();
  if count <= 5 {
    for i in 1..=count {
      page_numbers.push(i.to_string());
    }
  } else {
    if current > 3 {
      page_numbers.push("1".to_string());
      if current > 4 {
        page_numbers.push("...".to_string());
      }
    }
    let start = if current > 2 { current - 2 } else { 1 };
    let end = if current < count - 2 { current + 2 } else { count };
    for i in start..=end {
      page_numbers.push(i.to_string());
    }
    if current < count - 2 {
      if current < count - 3 {
        page_numbers.push("...".to_string());
      }
      page_numbers.push(count.to_string());
    }
  }
  page_numbers
}

#[hook]
pub(crate) fn use_pagination (params: PaginationHookParams)
 -> ( usize, Callback<usize>, Vec<String> ) {
  let PaginationHookParams { count, current, on_change } = params.clone();
  let current_page = use_state(|| current);

  let go_to_page = {
    let current_page = current_page.clone();
    Callback::from(move |page: usize| {
      if page > 0 && page <= count {
        current_page.set(page);
        on_change.emit(page);
      }
    })
  };

  let page_numbers = get_page_numbers(count, *current_page);

  (*current_page, go_to_page, page_numbers)
}
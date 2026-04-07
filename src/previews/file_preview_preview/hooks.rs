use web_sys::{File};
use yew::{hook, use_state, Callback};

#[hook]
pub fn use_file_preview_preview() -> (Vec<File>, Callback<Vec<File>>, Callback<u32>) {
  let files = use_state(|| Vec::new());

  let on_file_change = {
    let files = files.clone();
    Callback::from(move |file_list: Vec<File>| {
      files.set(file_list);
    })
  };

  let on_remove_file = {
    let files = files.clone();
    Callback::from(move |index: u32| {
      files.set(files.iter().enumerate().filter_map(|(i, file)| {
        if i as u32 != index {
          Some(file.clone())
        } else {
          None
        }
      }).collect());
    })
  };

  ((*files).clone(), on_file_change, on_remove_file)
}
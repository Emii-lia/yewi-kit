use web_sys::File;

#[derive(Clone, PartialEq)]
pub enum FileValue {
  File(File),
  Url(String, Option<String>, Option<String>),
}

impl FileValue {
  pub fn file(file: File) -> Self {
    FileValue::File(file)
  }
  pub fn url(url: String) -> Self {
    FileValue::Url(url, None, None)
  }
  pub fn with_name(self, name: String) -> Self {
    match self {
      FileValue::File(file) => FileValue::File(file),
      FileValue::Url(url, r#type, _) => FileValue::Url(url, r#type, Some(name)),
    }
  }
  
  pub fn with_type(self, r#type: String) -> Self {
    match self {
      FileValue::File(file) => FileValue::File(file),
      FileValue::Url(url, _, name) => FileValue::Url(url, Some(r#type), name),
    }
  }
}
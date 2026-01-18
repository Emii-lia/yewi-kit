use yew::{Callback, Children, Classes};

#[derive(Clone, Debug, PartialEq)]
pub enum ToastType {
  Success,
  Error,
  Default
}

#[derive(Clone, Debug, PartialEq)]
pub enum ToastPosition {
  TopLeft,
  TopCenter,
  TopRight,
  BottomLeft,
  BottomCenter,
  BottomRight,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ToastItem {
  pub id: usize,
  pub children: Children,
  pub position: ToastPosition,
  pub variant: ToastType,
  pub duration: usize,
  pub class: Classes,
}
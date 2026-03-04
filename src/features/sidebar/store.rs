use yew::{hook, use_context};
use crate::features::sidebar::provider::SidebarContext;

#[hook]
pub(crate) fn use_sidebar_store() -> SidebarContext {
  use_context::<SidebarContext>()
    .expect("use_sidebar_store must be used within SidebarProvider")
}
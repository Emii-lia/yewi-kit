use yew::{hook, use_context};
use crate::components::sidebar::provider::SidebarContextType;

#[hook]
pub fn use_sidebar_store() -> SidebarContextType {
  use_context::<SidebarContextType>()
    .expect("SidebarContext not found. Make sure you are using SidebarProvider.")
}
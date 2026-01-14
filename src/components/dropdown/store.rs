use yew::{hook, use_context};
use crate::components::dropdown::provider::DropDownContextType;

#[hook]
pub(crate) fn use_dropdown_store() -> DropDownContextType {
    use_context::<DropDownContextType>()
        .expect("use_carousel_store must be used within DropdownProvider")
}
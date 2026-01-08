use yew::{hook, use_context};
use crate::components::carousel::provider::CarouselContextType;

#[hook]
pub(crate) fn use_carousel_store() -> CarouselContextType {
    use_context::<CarouselContextType>()
        .expect("use_carousel_store must be used within CarouselProvider")
}
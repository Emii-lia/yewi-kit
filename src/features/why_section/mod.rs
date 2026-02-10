mod data;

use yew::{function_component, html, Html};
use crate::features::why_section::data::get_why_data;

#[function_component(WhySection)]
pub fn why_section() -> Html {
  let why_data = get_why_data();

  html! {
    <div class="WhySection">
      <div class="why-section-container">
        <div class="why-header">
          <h2 class="why-title">{"Why Yewi-kit"}</h2>
          <p class="why-subtitle">{"Components you control. Setup you don't think about."}</p>
        </div>
        <div class="why-grid">
          {for why_data.iter().map(|why| {
            html! {
              <div class="why-item" key={why.title.clone()}>
                <h3 class="why-item-title">{why.title.clone()}</h3>
                <p class="why-item-description">{why.description.clone()}</p>
              </div>
            }
          })}
        </div>
      </div>
    </div>
  }
}
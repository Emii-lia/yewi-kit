use yew::{html, Html, component};
use crate::features::ecosystem_template::EcosystemTemplate;
use crate::features::ecosystem_template::types::yewi_seo::EcosystemYewiSeo;

#[component(YewiSeoPage)]
pub(crate) fn yewi_seo_page() -> Html {
  html! {
    <div class="YewiSeoPage page-container">
      <EcosystemTemplate<EcosystemYewiSeo>/>
    </div>
  }
}
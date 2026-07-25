use yew::{html, Html, component};
use crate::features::ecosystem_template::EcosystemTemplate;
use crate::features::ecosystem_template::types::yewi_cli::EcosystemYewiCli;

#[component(YewiCliPage)]
pub(crate) fn yewi_cli_page() -> Html {
  html! {
    <div class="YewiCliPage page-container">
      <EcosystemTemplate<EcosystemYewiCli>/>
    </div>
  }
}
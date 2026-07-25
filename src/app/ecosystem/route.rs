use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum EcosystemRoute {
  #[at("/ecosystem/yewi-cli")]
  YewiCli,
  #[at("/ecosystem/yewi-seo")]
  YewiSeo,
  #[not_found]
  #[at("/ecosystem/404")]
  NotFound,
}

impl EcosystemRoute {
  pub fn iter() -> impl Iterator<Item = EcosystemRoute> {
    vec![
      EcosystemRoute::YewiCli,
      EcosystemRoute::YewiSeo,
      EcosystemRoute::NotFound
    ].into_iter()
  }
  
  pub fn to_string(&self) -> String {
    match self {
      EcosystemRoute::YewiCli => "Yewi CLI".to_string(),
      EcosystemRoute::YewiSeo => "Yewi SEO".to_string(),
      EcosystemRoute::NotFound => "Not Found".to_string()
    }
  }
}
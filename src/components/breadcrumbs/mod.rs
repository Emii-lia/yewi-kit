use yew::{classes, function_component, html, Classes, Html, Properties};
use yew_router::components::Link;
use yew_router::Routable;

#[derive(Properties, PartialEq ,Clone)]
pub struct BreadcrumbsProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or("/".into())]
  pub separator: String
}

#[function_component(Breadcrumbs)]
pub fn breadcrumbs(props: &BreadcrumbsProps) -> Html {
  html! {
    <div class={classes!("Breadcrumbs", &props.class)} style={format!("--separator: '{}';", props.separator)}>
      {props.children.clone()}
    </div>
  }
}

#[derive(Properties, PartialEq ,Clone)]
pub struct BreadCrumbProps<R>
where
  R: Routable
{
  pub href: R,
  pub children: Html,
  #[prop_or_default]
  pub class: Classes
}

#[function_component(BreadCrumb)]
pub fn bread_crumb<R>(props: &BreadCrumbProps<R>) -> Html
where
  R: Routable + 'static
{
  html! {
    <Link<R> to={props.href.clone()} classes={classes!("BreadCrumbs__item", &props.class)}>
      { props.children.clone()}
    </Link<R>>
  }
}
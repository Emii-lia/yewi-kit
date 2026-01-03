use yew::{classes, function_component, html, Classes, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct CardDescriptionProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
}
#[function_component(CardDescription)]
pub(crate) fn card_description(props: &CardDescriptionProps) -> Html {
  html! {
    <div
      class={classes!("Card__description", &props.class)}
      data-cy="card_description"
    >
      {props.children.clone()}
    </div>
  }
}

#[derive(Properties, Clone, PartialEq)]
pub struct  CardTitleProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
}
#[function_component(CardTitle)]
pub(crate) fn card_title(props: &CardTitleProps) -> Html {
  html! {
    <div
      class={classes!("Card__title", &props.class)}
      data-cy="card_title"
    >
      {props.children.clone()}
    </div>
  }
}

#[derive(Properties, Clone, PartialEq)]
pub struct CardFooterProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
}
#[function_component(CardFooter)]
pub(crate) fn card_footer(props: &CardFooterProps) -> Html {
  html! {
    <div
      class={classes!("Card__footer", &props.class)}
      data-cy="card_footer"
    >
      {props.children.clone()}
    </div>
  }
}

#[derive(Properties, Clone, PartialEq)]
pub struct CardContentProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
}
#[function_component(CardContent)]
pub(crate) fn card_content(props: &CardContentProps) -> Html {
  html! {
    <div
      class={classes!("Card__content", &props.class)}
      data-cy="card_content"
    >
      {props.children.clone()}
    </div>
  }
}

#[derive(Properties, Clone, PartialEq)]
pub struct CardHeaderProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
}
#[function_component(CardHeader)]
pub(crate) fn card_content(props: &CardHeaderProps) -> Html {
  html! {
    <div
      class={classes!("Card__header", &props.class)}
      data-cy="card_header"
    >
      {props.children.clone()}
    </div>
  }
}

#[derive(Properties, Clone, PartialEq)]
pub struct CardActionProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
}

#[function_component(CardAction)]
pub(crate) fn card_action(props: &CardActionProps) -> Html {
  html! {
    <div
      class={classes!("Card__action", &props.class)}
      data-cy="card_action"
    >
      {props.children.clone()}
    </div>
  }
}

#[derive(Properties, Clone, PartialEq)]
pub struct CardProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
}
#[function_component(Card)]
pub(crate) fn card(props: &CardProps) -> Html {
  html! {
    <div
      class={classes!("Card", &props.class)}
      data-cy="card"
    >
      {props.children.clone()}
    </div>
  }
}
mod types;

use yew::{classes, component, html, Children, Classes, Html, Properties};
pub(crate) use types::*;

#[derive(PartialEq, Clone, Properties)]
pub(crate) struct ChildrenWithClassProps {
  pub(crate) children: Children,
  #[prop_or_default]
  pub(crate) class: Classes,
}


#[derive(PartialEq, Clone, Properties)]
pub(crate) struct TableProps {
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(TableVariant::Default)]
  pub variant: TableVariant
}
#[component(Table)]
pub(crate) fn table(props: &TableProps) -> Html {
  let variant_class = format!("table-{:?}", props.variant).to_lowercase();

  html! {
    <table class={classes!("Table", variant_class, &props.class)}>
      {props.children.clone()}
    </table>
  }
}

#[component(TableHead)]
pub(crate) fn table_head(props: &ChildrenWithClassProps) -> Html {
  html! {
    <thead class={classes!("Table__head", &props.class)}>
      {props.children.clone()}
    </thead>
  }
}

#[component(TableBody)]
pub(crate) fn table_body(props: &ChildrenWithClassProps) -> Html {
  html! {
    <tbody class={classes!("Table__body", &props.class)}>
      {props.children.clone()}
    </tbody>
  }
}

#[component(TableRow)]
pub(crate) fn table_row(props: &ChildrenWithClassProps) -> Html {
  html! {
    <tr class={classes!("TableRow", &props.class)}>
      {props.children.clone()}
    </tr>
  }
}

#[component(TableHeaderCell)]
pub(crate) fn table_header_cell(props: &ChildrenWithClassProps) -> Html {
  html! {
    <th class={classes!("TableRow__header-cell", &props.class)}>
      {props.children.clone()}
    </th>
  }
}

#[component(TableDataCell)]
pub(crate) fn table_data_cell(props: &ChildrenWithClassProps) -> Html {
  html! {
    <td class={classes!("TableRow__data-cell", &props.class)}>
      {props.children.clone()}
    </td>
  }
}
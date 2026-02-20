pub mod types;

use yew::{function_component, html, Classes, Html, Properties};
use crate::components::{Table, TableBody, TableDataCell, TableHead, TableHeaderCell, TableRow};
use crate::features::prop_table::types::PropRow;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub props: Vec<PropRow>,
  #[prop_or_default]
  pub class: Classes,
}
#[function_component(PropTable)]
pub fn prop_table(props: &Props) -> Html {
  html! {
    <div class="PropTable">
      <h3 class="PropTable__title">{"Props"}</h3>
      <Table >
        <TableHead>
          <TableRow>
            <TableHeaderCell>{"Name"}</TableHeaderCell>
            <TableHeaderCell>{"Type"}</TableHeaderCell>
            <TableHeaderCell>{"Description"}</TableHeaderCell>
            <TableHeaderCell>{"Default"}</TableHeaderCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {for props.props.iter().map(|prop| html! {
            <TableRow key={prop.name.clone()}>
              <TableDataCell><p class="PropTable__item prop-name">{prop.name.clone()}</p></TableDataCell>
              <TableDataCell><p class="PropTable__item prop-type">{prop.r#type.clone()}</p></TableDataCell>
              <TableDataCell class={"prop-description"}><p class="PropTable__item">{prop.description.clone()}</p></TableDataCell>
              <TableDataCell><p class="PropTable__item prop-type">{
                prop.default.clone().unwrap_or_else(|| "".to_string())
              }</p></TableDataCell>
            </TableRow>
          })}
        </TableBody>
      </Table>
    </div>
  }
}
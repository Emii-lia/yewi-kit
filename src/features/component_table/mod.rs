pub mod types;

use yew::{function_component, html, Html, Properties};
use crate::components::{Table, TableBody, TableDataCell, TableHead, TableHeaderCell, TableRow};
use crate::features::component_table::types::ComponentRow;


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub components: Vec<ComponentRow>
}
#[function_component(ComponentTable)]
pub fn component_table(props: &Props) -> Html {
  html! {
    <div class="PropTable">
      <h3 class="PropTable__title">{"Components"}</h3>
      <Table >
        <TableHead>
          <TableRow>
            <TableHeaderCell>{"Name"}</TableHeaderCell>
            <TableHeaderCell>{"Description"}</TableHeaderCell>
            <TableHeaderCell>{"Props"}</TableHeaderCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {for props.components.iter().map(|prop| html! {
            <TableRow key={prop.name.clone()}>
              <TableDataCell><p class="PropTable__item prop-name">{prop.name.clone()}</p></TableDataCell>
              <TableDataCell class={"prop-description"}><p class="PropTable__item">{prop.description.clone()}</p></TableDataCell>
              <TableDataCell><p class="PropTable__item prop-type">{
                format!("[{}]", prop.props.clone().join(", "))
              }</p></TableDataCell>
            </TableRow>
          })}
        </TableBody>
      </Table>
    </div>
  }
}
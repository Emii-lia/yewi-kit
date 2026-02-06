use yew::{function_component, html, Html};
use crate::components::{CodePreview, Table, TableBody, TableDataCell, TableHead, TableHeaderCell, TableRow, TableVariant};
use crate::previews::PreviewContainer;

#[function_component(TablePreview)]
pub(crate) fn table_preview() -> Html {
  html! {
    <div  class="PreviewSection">
      <h1 class="preview-title">
        {"Table"}
      </h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display data in a structured table format."}
        </div>
        <CodePreview code={"yewi add table"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer title={"Default"} code={r#"
    <Table>
      <TableHead>
        <TableRow>
          <TableHeaderCell>{"Name"}</TableHeaderCell>
          <TableHeaderCell>{"Age"}</TableHeaderCell>
          <TableHeaderCell>{"Role"}</TableHeaderCell>
        </TableRow>
      </TableHead>
      <TableBody>
        <TableRow>
          <TableDataCell>{"Alice"}</TableDataCell>
          <TableDataCell>{"28"}</TableDataCell>
          <TableDataCell>{"Engineer"}</TableDataCell>
        </TableRow>
        <TableRow>
          <TableDataCell>{"Bob"}</TableDataCell>
          <TableDataCell>{"34"}</TableDataCell>
          <TableDataCell>{"Designer"}</TableDataCell>
        </TableRow>
        <TableRow>
          <TableDataCell>{"Charlie"}</TableDataCell>
          <TableDataCell>{"41"}</TableDataCell>
          <TableDataCell>{"Manager"}</TableDataCell>
        </TableRow>
      </TableBody>
    </Table>
          "#}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableHeaderCell>{"Name"}</TableHeaderCell>
                  <TableHeaderCell>{"Age"}</TableHeaderCell>
                  <TableHeaderCell>{"Role"}</TableHeaderCell>
                </TableRow>
              </TableHead>
              <TableBody>
                <TableRow>
                  <TableDataCell>{"Alice"}</TableDataCell>
                  <TableDataCell>{"28"}</TableDataCell>
                  <TableDataCell>{"Engineer"}</TableDataCell>
                </TableRow>
                <TableRow>
                  <TableDataCell>{"Bob"}</TableDataCell>
                  <TableDataCell>{"34"}</TableDataCell>
                  <TableDataCell>{"Designer"}</TableDataCell>
                </TableRow>
                <TableRow>
                  <TableDataCell>{"Charlie"}</TableDataCell>
                  <TableDataCell>{"41"}</TableDataCell>
                  <TableDataCell>{"Manager"}</TableDataCell>
                </TableRow>
              </TableBody>
            </Table>
          </PreviewContainer>
          <PreviewContainer title={"Striped"} code={r#"
    <Table variant={TableVariant::Striped}>
      <TableHead>
        <TableRow>
          <TableHeaderCell>{"Name"}</TableHeaderCell>
          <TableHeaderCell>{"Age"}</TableHeaderCell>
          <TableHeaderCell>{"Role"}</TableHeaderCell>
        </TableRow>
      </TableHead>
      <TableBody>
        <TableRow>
          <TableDataCell>{"Alice"}</TableDataCell>
          <TableDataCell>{"28"}</TableDataCell>
          <TableDataCell>{"Engineer"}</TableDataCell>
        </TableRow>
        <TableRow>
          <TableDataCell>{"Bob"}</TableDataCell>
          <TableDataCell>{"34"}</TableDataCell>
          <TableDataCell>{"Designer"}</TableDataCell>
        </TableRow>
        <TableRow>
          <TableDataCell>{"Charlie"}</TableDataCell>
          <TableDataCell>{"41"}</TableDataCell>
          <TableDataCell>{"Manager"}</TableDataCell>
        </TableRow>
      </TableBody>
    </Table>
          "#}>
            <Table variant={TableVariant::Striped}>
              <TableHead>
                <TableRow>
                  <TableHeaderCell>{"Name"}</TableHeaderCell>
                  <TableHeaderCell>{"Age"}</TableHeaderCell>
                  <TableHeaderCell>{"Role"}</TableHeaderCell>
                </TableRow>
              </TableHead>
              <TableBody>
                <TableRow>
                  <TableDataCell>{"Alice"}</TableDataCell>
                  <TableDataCell>{"28"}</TableDataCell>
                  <TableDataCell>{"Engineer"}</TableDataCell>
                </TableRow>
                <TableRow>
                  <TableDataCell>{"Bob"}</TableDataCell>
                  <TableDataCell>{"34"}</TableDataCell>
                  <TableDataCell>{"Designer"}</TableDataCell>
                </TableRow>
                <TableRow>
                  <TableDataCell>{"Charlie"}</TableDataCell>
                  <TableDataCell>{"41"}</TableDataCell>
                  <TableDataCell>{"Manager"}</TableDataCell>
                </TableRow>
              </TableBody>
            </Table>
          </PreviewContainer>
          <PreviewContainer title={"Default"} code={r#"
    <Table variant={TableVariant::Bordered}>
      <TableHead>
        <TableRow>
          <TableHeaderCell>{"Name"}</TableHeaderCell>
          <TableHeaderCell>{"Age"}</TableHeaderCell>
          <TableHeaderCell>{"Role"}</TableHeaderCell>
        </TableRow>
      </TableHead>
      <TableBody>
        <TableRow>
          <TableDataCell>{"Alice"}</TableDataCell>
          <TableDataCell>{"28"}</TableDataCell>
          <TableDataCell>{"Engineer"}</TableDataCell>
        </TableRow>
        <TableRow>
          <TableDataCell>{"Bob"}</TableDataCell>
          <TableDataCell>{"34"}</TableDataCell>
          <TableDataCell>{"Designer"}</TableDataCell>
        </TableRow>
        <TableRow>
          <TableDataCell>{"Charlie"}</TableDataCell>
          <TableDataCell>{"41"}</TableDataCell>
          <TableDataCell>{"Manager"}</TableDataCell>
        </TableRow>
      </TableBody>
    </Table>
          "#}>
            <Table variant={TableVariant::Bordered}>
              <TableHead>
                <TableRow>
                  <TableHeaderCell>{"Name"}</TableHeaderCell>
                  <TableHeaderCell>{"Age"}</TableHeaderCell>
                  <TableHeaderCell>{"Role"}</TableHeaderCell>
                </TableRow>
              </TableHead>
              <TableBody>
                <TableRow>
                  <TableDataCell>{"Alice"}</TableDataCell>
                  <TableDataCell>{"28"}</TableDataCell>
                  <TableDataCell>{"Engineer"}</TableDataCell>
                </TableRow>
                <TableRow>
                  <TableDataCell>{"Bob"}</TableDataCell>
                  <TableDataCell>{"34"}</TableDataCell>
                  <TableDataCell>{"Designer"}</TableDataCell>
                </TableRow>
                <TableRow>
                  <TableDataCell>{"Charlie"}</TableDataCell>
                  <TableDataCell>{"41"}</TableDataCell>
                  <TableDataCell>{"Manager"}</TableDataCell>
                </TableRow>
              </TableBody>
            </Table>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
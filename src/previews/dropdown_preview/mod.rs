use yew::{function_component, html, Html};
use crate::components::{Button, Dropdown, DropdownItem, DropdownMenu, DropdownPosition, DropdownTrigger};
use crate::previews::PreviewContainer;

#[function_component(DropdownPreview)]
pub(crate) fn dropdown_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Dropdown"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Use the Dropdown component to create a toggleable menu of options."}
        </div>
        <pre class="code-block">
          <code>
{"yewi add dropdown"}
          </code>
        </pre>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Bottom"}
            code={r#"
<Dropdown>
  <DropdownTrigger>
    <Button>{"Open Dropdown"}</Button>
  </DropdownTrigger>
  <DropdownMenu>
    <DropdownItem>{"Option 1"}</DropdownItem>
    <DropdownItem>{"Option 2"}</DropdownItem>
    <DropdownItem>{"Option 3"}</DropdownItem>
  </DropdownMenu>
</Dropdown>
            "#}
          >
            <Dropdown>
              <DropdownTrigger>
                <Button>{"Open Dropdown"}</Button>
              </DropdownTrigger>
              <DropdownMenu>
                <DropdownItem>{"Option 1"}</DropdownItem>
                <DropdownItem>{"Option 2"}</DropdownItem>
                <DropdownItem>{"Option 3"}</DropdownItem>
              </DropdownMenu>
            </Dropdown>
          </PreviewContainer>
          <PreviewContainer
            title={"Top"}
            code={r#"
<Dropdown>
  <DropdownTrigger>
    <Button>{"Open Dropdown"}</Button>
  </DropdownTrigger>
  <DropdownMenu position={DropdownPosition::Top}>
    <DropdownItem>{"Option 1"}</DropdownItem>
    <DropdownItem>{"Option 2"}</DropdownItem>
    <DropdownItem>{"Option 3"}</DropdownItem>
  </DropdownMenu>
</Dropdown>
          "#}
        >
          <Dropdown>
            <DropdownTrigger>
              <Button>{"Open Dropdown"}</Button>
            </DropdownTrigger>
            <DropdownMenu position={DropdownPosition::Top}>
              <DropdownItem>{"Option 1"}</DropdownItem>
              <DropdownItem>{"Option 2"}</DropdownItem>
              <DropdownItem>{"Option 3"}</DropdownItem>
            </DropdownMenu>
          </Dropdown>
        </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
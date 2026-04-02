mod data;

use yew::{component, html, Html};
use crate::components::{Pagination, CodePreview};
use crate::features::prop_table::PropTable;
use crate::previews::pagination_preview::data::get_props;
use crate::previews::PreviewContainer;
use crate::types::Size;
use crate::components::types::PaginationVariant;

#[component(PaginationPreview)]
pub(crate) fn pagination_preview() -> Html {
  let props = get_props();
  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">{"Pagination"}</h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Navigate through pages of content."}
        </div>
        <CodePreview code={"yewi add pagination"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
    <Pagination count={10} current={1}/>
            "#}
          >
            <Pagination count={10} current={1}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Outlined"}
            code={r#"
    <Pagination count={10} current={5} variant={PaginationVariant::Outlined}/>
            "#}
          >
            <Pagination
              count={10}
              current={5}
              variant={PaginationVariant::Outlined}
            />
          </PreviewContainer>
          <PreviewContainer
            title={"Rounded"}
            code={r#"
    <Pagination count={10} current={3} rounded=true />
            "#}
          >
            <Pagination
              count={10}
              current={3}
              rounded=true
            />
          </PreviewContainer>
          <PreviewContainer
            title={"Sizes"}
            code={r#"
    <Pagination count={10} current={1} size={Size::Small}/>
    <Pagination count={10} current={1} size={Size::Medium}/>
    <Pagination count={10} current={1} size={Size::Large}/>
            "#}
          >
            <Pagination count={10} current={1} size={Size::Small}/>
            <Pagination count={10} current={1} size={Size::Medium}/>
            <Pagination count={10} current={1} size={Size::Large}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Rounded Outlined"}
            code={r#"
    <Pagination count={10} current={7} variant={PaginationVariant::Outlined} rounded=true />
            "#}
          >
            <Pagination
              count={10}
              current={7}

              variant={PaginationVariant::Outlined}
              rounded=true
            />
          </PreviewContainer>
          <PreviewContainer
            title={"Many Pages"}
            code={r#"
    <Pagination count={50} current={25} />
            "#}
          >
            <Pagination
              count={50}
              current={25}
            />
          </PreviewContainer>
        </div>
      </div>
      <PropTable props={props}/>
    </div>
  }
}
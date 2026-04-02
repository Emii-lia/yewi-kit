mod data;

use yew::{component, html, Html};
use crate::app::docs::routes::DocsRoute;
use crate::components::{Breadcrumbs, BreadCrumb, CodePreview};
use crate::features::component_table::ComponentTable;
use crate::features::prop_table::PropTable;
use crate::previews::breadcrumbs_preview::data::{get_components, get_props};
use crate::previews::PreviewContainer;

#[component(BreadcrumbsPreview)]
pub(crate) fn breadcrumbs_preview() -> Html {
  let components = get_components();
  let props = get_props();

  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">{ "Breadcrumbs" }</h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display navigation hierarchy with links to parent pages."}
        </div>
        <CodePreview code={"yewi add breadcrumbs"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
    <Breadcrumbs>
      <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
        {"Home"}
      </BreadCrumb<DocsRoute>>
      <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
        {"Docs"}
      </BreadCrumb<DocsRoute>>
      <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
        {"Breadcrumbs"}
      </BreadCrumb<DocsRoute>>
    </Breadcrumbs>
            "#}
          >
            <Breadcrumbs>
              <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
                {"Home"}
              </BreadCrumb<DocsRoute>>
              <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
                {"Docs"}
              </BreadCrumb<DocsRoute>>
              <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
                {"Breadcrumbs"}
              </BreadCrumb<DocsRoute>>
            </Breadcrumbs>
          </PreviewContainer>
          <PreviewContainer
            title={"Separator"}
            code={r#"
    <Breadcrumbs class="bg-slate-100 rounded-md">
      <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
        {"Home"}
      </BreadCrumb<DocsRoute>>
      <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
        {"Components"}
      </BreadCrumb<DocsRoute>>
      <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage} class="text-primary-600">
        {"Current Page"}
      </BreadCrumb<DocsRoute>>
    </Breadcrumbs>
            "#}
          >
            <Breadcrumbs separator={">"}>
              <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
                {"Home"}
              </BreadCrumb<DocsRoute>>
              <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
                {"Components"}
              </BreadCrumb<DocsRoute>>
              <BreadCrumb<DocsRoute> href={DocsRoute::BreadcrumbsPage}>
                {"Current Page"}
              </BreadCrumb<DocsRoute>>
            </Breadcrumbs>
          </PreviewContainer>
        </div>
      </div>
      <ComponentTable components={components} />
      <PropTable props={props} />
    </div>
  }
}
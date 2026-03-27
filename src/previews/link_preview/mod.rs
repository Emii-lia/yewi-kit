pub mod data;

use yew::{function_component, html, Html};
use yew_icons::IconData;
use crate::app::docs::routes::DocsRoute;
use crate::components::CodePreview;
use crate::components::link::Link;
use crate::components::link::types::{LinkVariant};
use crate::features::prop_table::PropTable;
use crate::previews::link_preview::data::get_props;
use crate::previews::PreviewContainer;
use crate::types::Size;

#[function_component(LinkPreview)]
pub fn link_preview() -> Html {
  let props = get_props();
  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">{ "Link" }</h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Create navigational links to different routes in your application."}
        </div>
        <CodePreview code={"yewi add link"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
  <Link<DocsRoute> href={DocsRoute::AvatarPage}>
    {"View avatar"}
  </Link<DocsRoute>>
            "#}
          >
            <Link<DocsRoute> href={DocsRoute::AvatarPage}>
              {"View avatar"}
            </Link<DocsRoute>>
          </PreviewContainer>
          <PreviewContainer
            title={"Button"}
            code={r#"
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::primary()}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::secondary()}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::tertiary()}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::tertiary()}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::danger()}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::success()}
  >
    {"View avatar"}
  </Link<DocsRoute>>
            "#}
          >
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::primary()}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::secondary()}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::tertiary()}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::tertiary()}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::danger()}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::success()}
            >
              {"View avatar"}
            </Link<DocsRoute>>
          </PreviewContainer>
          <PreviewContainer
            title={"Size"}
            code={r#"
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::primary().with_size(Size::Small)}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::primary().with_size(Size::Medium)}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::primary().with_size(Size::Large)}
  >
    {"View avatar"}
  </Link<DocsRoute>>
            "#}
          >
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::primary().with_size(Size::Small)}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::primary().with_size(Size::Medium)}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::primary().with_size(Size::Large)}
            >
              {"View avatar"}
            </Link<DocsRoute>>
          </PreviewContainer>
          <PreviewContainer
            title={"Icon"}
            code={r#"
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    icon={IconData::LUCIDE_USER}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::primary()}
    icon={IconData::LUCIDE_USER}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::secondary()}
    icon={IconData::LUCIDE_USER}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::tertiary()}
    icon={IconData::LUCIDE_USER}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::danger()}
    icon={IconData::LUCIDE_USER}
  >
    {"View avatar"}
  </Link<DocsRoute>>
  <Link<DocsRoute>
    href={DocsRoute::AvatarPage}
    variant={LinkVariant::success()}
    icon={IconData::LUCIDE_USER}
  >
    {"View avatar"}
  </Link<DocsRoute>>
            "#}
          >
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              icon={IconData::LUCIDE_USER}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::primary()}
              icon={IconData::LUCIDE_USER}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::secondary()}
              icon={IconData::LUCIDE_USER}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::tertiary()}
              icon={IconData::LUCIDE_USER}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::danger()}
              icon={IconData::LUCIDE_USER}
            >
              {"View avatar"}
            </Link<DocsRoute>>
            <Link<DocsRoute>
              href={DocsRoute::AvatarPage}
              variant={LinkVariant::success()}
              icon={IconData::LUCIDE_USER}
            >
              {"View avatar"}
            </Link<DocsRoute>>
          </PreviewContainer>
        </div>
      </div>
      <PropTable props={props} />
    </div>
  }
}
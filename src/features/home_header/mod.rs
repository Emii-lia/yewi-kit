use yew::{classes, component, html, Html};
use yew_icons::IconData;
use yew_router::prelude::Link as RouterLink;
use crate::app::docs::routes::DocsRoute;
use crate::app::routes::AppRoute;
use crate::components::{Button, ButtonVariant, link::Link};
use crate::components::link::types::LinkVariant;
use crate::types::Size;

#[component(HomeHeader)]
pub fn home_header() -> Html {
  html! {
    <header class="HomeHeader">
      <nav class="HomeHeader__nav">
        <RouterLink<AppRoute>
          classes={classes!("HomeHeader__nav--logo ")}
          to={AppRoute::Home}
        >
          <div
            class="nav-logo"
            style={"--logo: url('/icons/logo.png');"}
          />
          <h1 class="nav-title">{"Yewi-kit"}</h1>
        </RouterLink<AppRoute>>
        <div class="HomeHeader__nav--actions">
          <Link<DocsRoute>
            href={DocsRoute::Docs}
            icon={IconData::LUCIDE_BOOK_OPEN}
            variant={LinkVariant::tertiary().with_size(Size::Small)}
          >
            {"Documentation"}
          </Link<DocsRoute>>
          <Button
            icon={IconData::LUCIDE_GITHUB}
            variant={ButtonVariant::Tertiary}
            size={Size::Small}
            href="https://github.com/Emii-lia/yewi-kit"
          >
            {"Github"}
          </Button>
        </div>
        <div class="HomeHeader__nav--actions-md">
          <Link<DocsRoute>
            href={DocsRoute::Docs}
            icon={IconData::LUCIDE_BOOK_OPEN}
            variant={LinkVariant::tertiary().with_size(Size::Small)}
          />
          <Button
            icon={IconData::LUCIDE_GITHUB}
            variant={ButtonVariant::Tertiary}
            size={Size::Small}
            href="https://github.com/Emii-lia/yewi-kit"
          />
        </div>
      </nav>
    </header>
  }
}
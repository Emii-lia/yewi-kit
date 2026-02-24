use yew::{classes, function_component, html, Html};
use yew_icons::IconData;
use yew_router::prelude::Link;
use crate::app::docs::routes::DocsRoute;
use crate::app::routes::AppRoute;
use crate::components::{Button, ButtonVariant};
use crate::types::Size;

#[function_component(HomeHeader)]
pub fn home_header() -> Html {
  html! {
    <header class="HomeHeader">
      <nav class="HomeHeader__nav">
        <Link<AppRoute>
          classes={classes!("HomeHeader__nav--logo ")}
          to={AppRoute::Home}
        >
          <span class="nav-logo">{"Y"}</span>
          <h1 class="nav-title">{"Yewi-kit"}</h1>
        </Link<AppRoute>>
        <div class="HomeHeader__nav--actions">
          <Link<DocsRoute>
            to={DocsRoute::Docs}
            classes={classes!("nav-action")}
          >
            <Button
              icon={IconData::LUCIDE_BOOK_OPEN}
              variant={ButtonVariant::Tertiary}
            size={Size::Small}
            >
              {"Documentation"}
            </Button>
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
            to={DocsRoute::Docs}
            classes={classes!("nav-action")}
          >
            <Button
              icon={IconData::LUCIDE_BOOK_OPEN}
              variant={ButtonVariant::Tertiary}
            size={Size::Small}
            />
          </Link<DocsRoute>>
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
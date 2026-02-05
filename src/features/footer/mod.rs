use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
  html! {
    <footer class="Footer">
      <div class="Footer__container">
        <div class="footer-left">
          <p class="footer-cp">{'\u{00A9}'}{" 2026 Yewi-kit. Built for Rust developers."}</p>
        </div>
        <div class="footer-right">
          <a
            class="footer-link"
            href={"https://github.com/Emii-lia/yewi-kit?tab=MIT-1-ov-file"}
            target="_blank"
            rel="noreferrer"
          >
            {"License"}
          </a>
          <a
            class="footer-link"
            href={"https://github.com/Emii-lia/yewi-kit"}
            target="_blank"
            rel="noreferrer"
          >
            {"Github"}
          </a>
          <a
            class="footer-link"
            href={"https://github.com/Emii-lia/yewi-kit/issues"}
            target="_blank"
            rel="noreferrer"
          >
            {"Issues"}
          </a>
        </div>
      </div>
    </footer>
  }
}
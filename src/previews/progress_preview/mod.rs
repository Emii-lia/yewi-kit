use yew::{function_component, html, Html};
use crate::components::Progress;
use crate::previews::PreviewContainer;
use crate::types::Color;

#[function_component(ProgressPreview)]
pub(crate) fn progress_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">
        {"Progress"}
      </h1>
      <div class="preview-list">
        <PreviewContainer title={"Default"}>
          <Progress value={25} max={100} show_percentage=true />
          <Progress value={50} max={100} color={Color::Green} show_percentage=true />
          <Progress value={75} max={100} color={Color::Orange} show_percentage=true />
          <Progress value={100} max={100} color={Color::Red} show_percentage=true />
        </PreviewContainer>
        <PreviewContainer title={"Custom Height"}>
          <Progress value={40} max={100} height={10} show_percentage=true />
          <Progress value={70} max={100} color={Color::Purple} height={20} show_percentage=true />
          <Progress value={90} max={100} color={Color::Teal} height={30} show_percentage=true />
        </PreviewContainer>
        <PreviewContainer title={"Without Percentage"}>
          <Progress value={60} max={100} />
          <Progress value={80} max={100} color={Color::Pink} />
        </PreviewContainer>
        <PreviewContainer title={"Other colors"}>
          <Progress value={30} max={100} color={Color::Amber} show_percentage=true />
          <Progress value={55} max={100} color={Color::Sky} show_percentage=true />
          <Progress value={85} max={100} color={Color::Lime} show_percentage=true />
        </PreviewContainer>
      </div>
    </div>
  }
}
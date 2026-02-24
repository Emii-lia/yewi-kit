mod data;

use yew::{function_component, html, Html};
use crate::components::{CodePreview, Progress, ProgressVariant};
use crate::features::prop_table::PropTable;
use crate::previews::PreviewContainer;
use crate::previews::progress_preview::data::get_props;
use crate::types::{Color, Size};

#[function_component(ProgressPreview)]
pub(crate) fn progress_preview() -> Html {
  let props = get_props();

  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">
        {"Progress"}
      </h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display progress bars and indicators to visualize task completion status."}
        </div>
        <CodePreview code={"yewi add progress"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
    <Progress value={25} max={100} show_percentage=true />
    <Progress value={50} max={100} color={Color::Purple} show_percentage=true />
    <Progress value={75} max={100} color={Color::Teal} show_percentage=true />
    <Progress value={100} max={100} color={Color::Blue} show_percentage=true />
            "#}
          >
            <Progress value={25} max={100} show_percentage=true />
            <Progress value={50} max={100} color={Color::Purple} show_percentage=true />
            <Progress value={75} max={100} color={Color::Teal} show_percentage=true />
            <Progress value={100} max={100} color={Color::Blue} show_percentage=true />
          </PreviewContainer>
          <PreviewContainer
            title={"Radial"}
            code={r#"
    <Progress variant={ProgressVariant::Circular} radial_size={Size::Small} value={25} max={100} show_percentage=true />
    <Progress variant={ProgressVariant::Circular} radial_size={Size::Medium} value={50} max={100} color={Color::Purple} show_percentage=true />
    <Progress variant={ProgressVariant::Circular} radial_size={Size::Large} value={75} max={100} color={Color::Teal} show_percentage=true />
            "#}
          >
            <Progress variant={ProgressVariant::Circular} radial_size={Size::Small} value={25} max={100} show_percentage=true />
            <Progress variant={ProgressVariant::Circular} radial_size={Size::Medium} value={50} max={100} color={Color::Purple} show_percentage=true />
            <Progress variant={ProgressVariant::Circular} radial_size={Size::Large} value={75} max={100} color={Color::Teal} show_percentage=true />
          </PreviewContainer>
          <PreviewContainer
            title={"Custom Height"}
            code={r#"
    <Progress value={40} max={100} height={10} show_percentage=true />
    <Progress value={70} max={100} color={Color::Purple} height={20} show_percentage=true />
    <Progress value={90} max={100} color={Color::Teal} height={30} show_percentage=true />
            "#}
          >
            <Progress value={40} max={100} height={10} show_percentage=true />
            <Progress value={70} max={100} color={Color::Purple} height={20} show_percentage=true />
            <Progress value={90} max={100} color={Color::Teal} height={30} show_percentage=true />
          </PreviewContainer>
          <PreviewContainer
            title={"As fraction"}
            code={r#"
    <Progress variant={ProgressVariant::Circular} radial_size={Size::Medium} value={50} max={100} show_percentage=true as_fraction=true />
    <Progress value={75} max={100} show_percentage=true as_fraction=true />
            "#}
          >
            <Progress variant={ProgressVariant::Circular} radial_size={Size::Medium} value={50} max={100} show_percentage=true as_fraction=true />
            <Progress value={75} max={100} show_percentage=true as_fraction=true />
          </PreviewContainer>
          <PreviewContainer
            title={"Without Percentage"}
            code={r#"
    <Progress variant={ProgressVariant::Circular} value={60} max={100} />
    <Progress value={80} max={100} color={Color::Pink} />
            "#}
          >
            <Progress variant={ProgressVariant::Circular} value={60} max={100} />
            <Progress value={80} max={100}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Colours"}
            code={r#"
    <Progress value={30} max={100} color={Color::Amber} show_percentage=true />
    <Progress value={55} max={100} color={Color::Sky} show_percentage=true />
    <Progress value={85} max={100} color={Color::Lime} show_percentage=true />
            "#}
          >
            <Progress value={30} max={100} color={Color::Amber} show_percentage=true />
            <Progress value={55} max={100} color={Color::Sky} show_percentage=true />
            <Progress value={85} max={100} color={Color::Lime} show_percentage=true />
          </PreviewContainer>
        </div>
      </div>
      <PropTable props={props}/>
    </div>
  }
}
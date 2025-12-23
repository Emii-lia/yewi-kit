use yew::{function_component, html, Html};
use crate::components::Avatar;
use crate::previews::PreviewContainer;
use crate::types::{Color, Size};

#[function_component(AvatarPreview)]
pub(crate) fn avatar_preview() -> Html{
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Avatar"}</h1>
      <div class="preview-list">
        <PreviewContainer title={"Default"}>
          <Avatar alt={"John Doe"} size={Size::Small}/>
          <Avatar alt={"John Doe"} size={Size::Medium}/>
          <Avatar alt={"John Doe"} size={Size::Large}/>
        </PreviewContainer>
        <PreviewContainer title={"Rounded"}>
          <Avatar alt={"John Doe"} size={Size::Small} rounded=true/>
          <Avatar alt={"John Doe"} size={Size::Medium} rounded=true/>
          <Avatar alt={"John Doe"} size={Size::Large} rounded=true/>
        </PreviewContainer>
        <PreviewContainer title={"With Border"}>
          <Avatar alt={"John Doe"} size={Size::Small} rounded=true with_border=true/>
          <Avatar alt={"John Doe"} size={Size::Medium} rounded=true with_border=true/>
          <Avatar alt={"John Doe"} size={Size::Large} rounded=true with_border=true/>
        </PreviewContainer>
        <PreviewContainer title={"With image"}>
          <Avatar alt={"John Doe"} size={Size::Small} rounded=true src={"https://picsum.photos/200"}/>
          <Avatar alt={"John Doe"} size={Size::Medium} rounded=true src={"https://picsum.photos/200"} />
          <Avatar alt={"John Doe"} size={Size::Large} rounded=true src={"https://picsum.photos/200"}/>
        </PreviewContainer>
        <PreviewContainer title={"Colours"}>
          <Avatar alt={"John Doe"} size={Size::Medium} with_border=true rounded=true src={"https://picsum.photos/200"} color={Color::Red}/>
          <Avatar alt={"John Doe"} size={Size::Medium} rounded=true color={Color::Blue} />
          <Avatar alt={"John Doe"} size={Size::Medium} with_border=true src={"https://picsum.photos/200"} color={Color::Sky}/>
          <Avatar alt={"John Doe"} size={Size::Medium} color={Color::Rose} />
          <Avatar alt={"John Doe"} size={Size::Medium} with_border=true rounded=true color={Color::Amber} />
          <Avatar alt={"John Doe"} size={Size::Medium} with_border=true color={Color::Teal} />
        </PreviewContainer>
      </div>
    </div>
  }
}
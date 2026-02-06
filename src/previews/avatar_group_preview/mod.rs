use yew::{function_component, html, Html};
use crate::components::{Avatar, AvatarGroup, AvatarGroupVariant, CodePreview};
use crate::previews::PreviewContainer;
use crate::types::{ Size };

#[function_component(AvatarGroupPreview)]
pub(crate) fn avatar_group_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"AvatarGroup"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display a collection of avatars in linear or stacked layouts."}
        </div>
        <CodePreview code={"yewi add avatar avatar_group"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
    <AvatarGroup rounded=true size={Size::Small}>
      <Avatar alt={"John Doe"} src={"https://picsum.photos/200"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/201"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/202"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/205"} />
    </AvatarGroup>
    <AvatarGroup rounded=true>
      <Avatar alt={"John Doe"} src={"https://picsum.photos/200"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/201"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/202"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/205"} />
    </AvatarGroup>
    <AvatarGroup rounded=true size={Size::Large}>
      <Avatar alt={"John Doe"} src={"https://picsum.photos/200"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/201"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/202"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/205"} />
    </AvatarGroup>
            "#}
          >
            <AvatarGroup rounded=true size={Size::Small}>
              <Avatar alt={"John Doe"} src={"https://picsum.photos/200"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/201"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/202"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/205"} />
            </AvatarGroup>
            <AvatarGroup rounded=true>
              <Avatar alt={"John Doe"} src={"https://picsum.photos/200"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/201"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/202"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/205"} />
            </AvatarGroup>
            <AvatarGroup rounded=true size={Size::Large}>
              <Avatar alt={"John Doe"} src={"https://picsum.photos/200"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/201"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/202"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/205"} />
            </AvatarGroup>
          </PreviewContainer>
          <PreviewContainer
            title={"Stacked"}
            class="!gap-8"
            code={r#"
    <AvatarGroup rounded=true variant={AvatarGroupVariant::Stacked} size={Size::Small}>
      <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
    </AvatarGroup>
    <AvatarGroup rounded=true variant={AvatarGroupVariant::Stacked}>
      <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
    </AvatarGroup>
    <AvatarGroup rounded=true variant={AvatarGroupVariant::Stacked} size={Size::Large}>
      <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
      <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
    </AvatarGroup>
            "#}
          >
            <AvatarGroup rounded=true variant={AvatarGroupVariant::Stacked} size={Size::Small}>
              <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
            </AvatarGroup>
            <AvatarGroup rounded=true variant={AvatarGroupVariant::Stacked}>
              <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
            </AvatarGroup>
            <AvatarGroup rounded=true variant={AvatarGroupVariant::Stacked} size={Size::Large}>
              <Avatar alt={"John Doe"} src={"https://picsum.photos/203"} />
              <Avatar alt={"John Doe"} src={"https://picsum.photos/204"} />
            </AvatarGroup>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
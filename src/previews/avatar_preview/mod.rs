use yew::{function_component, html, Html};
use crate::components::Avatar;
use crate::previews::PreviewContainer;
use crate::types::{Color, Size};

#[function_component(AvatarPreview)]
pub(crate) fn avatar_preview() -> Html{
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Avatar"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display user profile pictures or initials."}
        </div>
        <pre class="code-block">
          <code>
    {"yewi add avatar"}
          </code>
        </pre>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
    <Avatar alt={"John Doe"} size={Size::Small}/>
    <Avatar alt={"John Doe"} size={Size::Medium}/>
    <Avatar alt={"John Doe"} size={Size::Large}/>
            "#}
          >
            <Avatar alt={"John Doe"} size={Size::Small}/>
            <Avatar alt={"John Doe"} size={Size::Medium}/>
            <Avatar alt={"John Doe"} size={Size::Large}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Rounded"}
            code={r#"
    <Avatar alt={"John Doe"} size={Size::Small} rounded=true/>
    <Avatar alt={"John Doe"} size={Size::Medium} rounded=true/>
    <Avatar alt={"John Doe"} size={Size::Large} rounded=true/>
            "#}
          >
            <Avatar alt={"John Doe"} size={Size::Small} rounded=true/>
            <Avatar alt={"John Doe"} size={Size::Medium} rounded=true/>
            <Avatar alt={"John Doe"} size={Size::Large} rounded=true/>
          </PreviewContainer>
          <PreviewContainer
            title={"With Border"}
            code={r#"
    <Avatar alt={"John Doe"} size={Size::Small} rounded=true with_border=true/>
    <Avatar alt={"John Doe"} size={Size::Medium} rounded=true with_border=true/>
    <Avatar alt={"John Doe"} size={Size::Large} rounded=true with_border=true/>
            "#}
          >
            <Avatar alt={"John Doe"} size={Size::Small} rounded=true with_border=true/>
            <Avatar alt={"John Doe"} size={Size::Medium} rounded=true with_border=true/>
            <Avatar alt={"John Doe"} size={Size::Large} rounded=true with_border=true/>
          </PreviewContainer>
          <PreviewContainer
            title={"With image"}
            code={r#"
    <Avatar alt={"John Doe"} size={Size::Small} rounded=true src={"https://picsum.photos/200"}/>
    <Avatar alt={"John Doe"} size={Size::Medium} rounded=true src={"https://picsum.photos/200"} />
    <Avatar alt={"John Doe"} size={Size::Large} rounded=true src={"https://picsum.photos/200"}/>
            "#}
          >
            <Avatar alt={"John Doe"} size={Size::Small} rounded=true src={"https://picsum.photos/200"}/>
            <Avatar alt={"John Doe"} size={Size::Medium} rounded=true src={"https://picsum.photos/200"} />
            <Avatar alt={"John Doe"} size={Size::Large} rounded=true src={"https://picsum.photos/200"}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Colours"}
            code={r#"
    <Avatar alt={"John Doe"} size={Size::Medium} with_border=true rounded=true src={"https://picsum.photos/200"} color={Color::Red}/>
    <Avatar alt={"John Doe"} size={Size::Medium} rounded=true color={Color::Blue} />
    <Avatar alt={"John Doe"} size={Size::Medium} with_border=true src={"https://picsum.photos/200"} color={Color::Sky}/>
    <Avatar alt={"John Doe"} size={Size::Medium} color={Color::Rose} />
    <Avatar alt={"John Doe"} size={Size::Medium} with_border=true rounded=true color={Color::Amber} />
    <Avatar alt={"John Doe"} size={Size::Medium} with_border=true color={Color::Teal} />
            "#}
          >
            <Avatar alt={"John Doe"} size={Size::Medium} with_border=true rounded=true src={"https://picsum.photos/200"} color={Color::Red}/>
            <Avatar alt={"John Doe"} size={Size::Medium} rounded=true color={Color::Blue} />
            <Avatar alt={"John Doe"} size={Size::Medium} with_border=true src={"https://picsum.photos/200"} color={Color::Sky}/>
            <Avatar alt={"John Doe"} size={Size::Medium} color={Color::Rose} />
            <Avatar alt={"John Doe"} size={Size::Medium} with_border=true rounded=true color={Color::Amber} />
            <Avatar alt={"John Doe"} size={Size::Medium} with_border=true color={Color::Teal} />
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
use yew::{function_component, html, Html};
use crate::components::{Avatar, Badge, BadgeColor, Button, Tab, Tabs};
use crate::previews::PreviewContainer;
use crate::types::{Color, Size};

#[function_component(TabsPreview)]
pub(crate) fn tabs_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">
        {"Tabs"}
      </h1>
      <div class="preview-list">
        <PreviewContainer title={"Default"}>
          <Tabs>
            <Tab
              label={"Button"}
              value={"button"}
            >
              <PreviewContainer title="Primary">
                <Button size={Size::Small}>
                  {"Primary"}
                </Button>
                <Button size={Size::Medium}>
                  {"Primary"}
                </Button>
                <Button size={Size::Large}>
                  {"Primary"}
                </Button>
              </PreviewContainer>
            </Tab>
            <Tab
              label={"Label"}
              value={"label"}
            >
              <PreviewContainer title={"Default"}>
                <Badge label="Default" color={BadgeColor::Red}/>
                <Badge label="Default" color={BadgeColor::Blue}/>
                <Badge label="Default" color={BadgeColor::Green}/>
                <Badge label="Default" color={BadgeColor::Yellow}/>
              </PreviewContainer>
            </Tab>
            <Tab
              label={"Avatar"}
              value={"avatar"}
            >
              <PreviewContainer title={"Default"}>
                <Avatar alt={"John Doe"} size={Size::Small}/>
                <Avatar alt={"John Doe"} size={Size::Medium}/>
                <Avatar alt={"John Doe"} size={Size::Large}/>
              </PreviewContainer>
            </Tab>
          </Tabs>
        </PreviewContainer>
        <PreviewContainer title={"Colour"}>
          <Tabs color={Color::Rose}>
            <Tab
              label={"Button"}
              value={"button"}
            >
              <PreviewContainer title="Primary">
                <Button size={Size::Small}>
                  {"Primary"}
                </Button>
                <Button size={Size::Medium}>
                  {"Primary"}
                </Button>
                <Button size={Size::Large}>
                  {"Primary"}
                </Button>
              </PreviewContainer>
            </Tab>
            <Tab
              label={"Label"}
              value={"label"}
            >
              <PreviewContainer title={"Default"}>
                <Badge label="Default" color={BadgeColor::Red}/>
                <Badge label="Default" color={BadgeColor::Blue}/>
                <Badge label="Default" color={BadgeColor::Green}/>
                <Badge label="Default" color={BadgeColor::Yellow}/>
              </PreviewContainer>
            </Tab>
            <Tab
              label={"Avatar"}
              value={"avatar"}
            >
              <PreviewContainer title={"Default"}>
                <Avatar alt={"John Doe"} size={Size::Small}/>
                <Avatar alt={"John Doe"} size={Size::Medium}/>
                <Avatar alt={"John Doe"} size={Size::Large}/>
              </PreviewContainer>
            </Tab>
          </Tabs>
        </PreviewContainer>
      </div>
    </div>
  }
}
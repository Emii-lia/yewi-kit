use yew::{function_component, html, Html, Properties};
use crate::components::{Carousel, CarouselContent, CarouselControls, CarouselItem, CarouselWrapper, CodePreview};
use crate::previews::PreviewContainer;

#[derive(Properties, PartialEq, Clone)]
struct Props {
  pub url: String,
  #[prop_or_default]
  pub alt: String,
}

#[function_component(ImageSlide)]
fn image_slide(props: &Props) -> Html {
  html! {
    <div class="flex justify-center items-center w-full h-full bg-gray-200">
      <img
        class="w-auto h-52 object-cover aspect-video rounded-lg"
        src={props.url.clone()}
        alt={props.alt.clone()}
      />
    </div>
  }
}
#[function_component(CarouselPreview)]
pub(crate) fn carousel_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">{ "Carousel" }</h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Create interactive slideshow."}
        </div>
        <CodePreview code={"yewi add carousel"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
    <Carousel>
      <CarouselWrapper>
        <CarouselContent>
          <CarouselItem id={"slide1".to_string()}>
            <ImageSlide url={"https://picsum.photos/200".to_string()} alt={"Slide 1".to_string()}/>
          </CarouselItem>
          <CarouselItem id={"slide2".to_string()}>
            <ImageSlide url={"https://picsum.photos/201".to_string()} alt={"Slide 2".to_string()}/>
          </CarouselItem>
          <CarouselItem id={"slide3".to_string()}>
            <ImageSlide url={"https://picsum.photos/202".to_string()} alt={"Slide 3".to_string()}/>
          </CarouselItem>
          <CarouselItem id={"slide4".to_string()}>
            <ImageSlide url={"https://picsum.photos/203".to_string()} alt={"Slide 4".to_string()}/>
          </CarouselItem>
        </CarouselContent>
        <CarouselControls/>
      </CarouselWrapper>
    </Carousel>
            "#}
          >
            <Carousel>
              <CarouselWrapper>
                <CarouselContent>
                  <CarouselItem id={"slide1".to_string()}>
                    <ImageSlide url={"https://picsum.photos/200".to_string()} alt={"Slide 1".to_string()}/>
                  </CarouselItem>
                  <CarouselItem id={"slide2".to_string()}>
                    <ImageSlide url={"https://picsum.photos/201".to_string()} alt={"Slide 2".to_string()}/>
                  </CarouselItem>
                  <CarouselItem id={"slide3".to_string()}>
                    <ImageSlide url={"https://picsum.photos/202".to_string()} alt={"Slide 3".to_string()}/>
                  </CarouselItem>
                  <CarouselItem id={"slide4".to_string()}>
                    <ImageSlide url={"https://picsum.photos/203".to_string()} alt={"Slide 4".to_string()}/>
                  </CarouselItem>
                </CarouselContent>
                <CarouselControls/>
              </CarouselWrapper>
            </Carousel>
          </PreviewContainer>
          <PreviewContainer
            title={"Auto Play"}
            code={r#"
   <Carousel auto_play=true auto_play_interval={2000.0}>
    <CarouselWrapper>
      <CarouselContent>
        <CarouselItem id={"slide1".to_string()}>
          <ImageSlide url={"https://picsum.photos/200".to_string()} alt={"Slide 1".to_string()}/>
        </CarouselItem>
        <CarouselItem id={"slide2".to_string()}>
          <ImageSlide url={"https://picsum.photos/201".to_string()} alt={"Slide 2".to_string()}/>
        </CarouselItem>
        <CarouselItem id={"slide3".to_string()}>
          <ImageSlide url={"https://picsum.photos/202".to_string()} alt={"Slide 3".to_string()}/>
        </CarouselItem>
        <CarouselItem id={"slide4".to_string()}>
          <ImageSlide url={"https://picsum.photos/203".to_string()} alt={"Slide 4".to_string()}/>
        </CarouselItem>
      </CarouselContent>
      <CarouselControls/>
    </CarouselWrapper>
  </Carousel>
            "#}
          >
            <Carousel auto_play=true auto_play_interval={2000.0}>
              <CarouselWrapper>
                <CarouselContent>
                  <CarouselItem id={"slide1".to_string()}>
                    <ImageSlide url={"https://picsum.photos/200".to_string()} alt={"Slide 1".to_string()}/>
                  </CarouselItem>
                  <CarouselItem id={"slide2".to_string()}>
                    <ImageSlide url={"https://picsum.photos/201".to_string()} alt={"Slide 2".to_string()}/>
                  </CarouselItem>
                  <CarouselItem id={"slide3".to_string()}>
                    <ImageSlide url={"https://picsum.photos/202".to_string()} alt={"Slide 3".to_string()}/>
                  </CarouselItem>
                  <CarouselItem id={"slide4".to_string()}>
                    <ImageSlide url={"https://picsum.photos/203".to_string()} alt={"Slide 4".to_string()}/>
                  </CarouselItem>
                </CarouselContent>
                <CarouselControls/>
              </CarouselWrapper>
            </Carousel>
          </PreviewContainer>
          <PreviewContainer
            title={"No Controls"}
            code={r#"
    <Carousel auto_play=true auto_play_interval={2000.0}>
      <CarouselWrapper>
        <CarouselContent>
          <CarouselItem id={"slide1".to_string()}>
            <ImageSlide url={"https://picsum.photos/200".to_string()} alt={"Slide 1".to_string()}/>
          </CarouselItem>
          <CarouselItem id={"slide2".to_string()}>
            <ImageSlide url={"https://picsum.photos/201".to_string()} alt={"Slide 2".to_string()}/>
          </CarouselItem>
          <CarouselItem id={"slide3".to_string()}>
            <ImageSlide url={"https://picsum.photos/202".to_string()} alt={"Slide 3".to_string()}/>
          </CarouselItem>
          <CarouselItem id={"slide4".to_string()}>
            <ImageSlide url={"https://picsum.photos/203".to_string()} alt={"Slide 4".to_string()}/>
          </CarouselItem>
        </CarouselContent>
        <CarouselControls show_arrows=false show_dots=false/>
      </CarouselWrapper>
    </Carousel>
            "#}
          >
            <Carousel auto_play=true auto_play_interval={2000.0}>
              <CarouselWrapper>
                <CarouselContent>
                  <CarouselItem id={"slide1".to_string()}>
                    <ImageSlide url={"https://picsum.photos/200".to_string()} alt={"Slide 1".to_string()}/>
                  </CarouselItem>
                  <CarouselItem id={"slide2".to_string()}>
                    <ImageSlide url={"https://picsum.photos/201".to_string()} alt={"Slide 2".to_string()}/>
                  </CarouselItem>
                  <CarouselItem id={"slide3".to_string()}>
                    <ImageSlide url={"https://picsum.photos/202".to_string()} alt={"Slide 3".to_string()}/>
                  </CarouselItem>
                  <CarouselItem id={"slide4".to_string()}>
                    <ImageSlide url={"https://picsum.photos/203".to_string()} alt={"Slide 4".to_string()}/>
                  </CarouselItem>
                </CarouselContent>
                <CarouselControls show_arrows=false show_dots=false/>
              </CarouselWrapper>
            </Carousel>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
use yew::{function_component, html, Html};
use crate::components::{Button, Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input};
use crate::previews::PreviewContainer;
use crate::types::ButtonVariant;

#[function_component(CardPreview)]
pub(crate) fn card_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{ "Card" }</h1>
      <div class="preview-list">
        <PreviewContainer
          title={"Basic Card"}
          code={r#"
            <Card>
              <CardHeader>
                <CardTitle>{"Login to your account"}</CardTitle>
                <CardDescription>
                  {"Enter your email below to login to your account"}
                </CardDescription>
                <CardAction>
                  <Button href={"/card#"} variant={ButtonVariant::Secondary}>{"Sign Up"}</Button>
                </CardAction>
              </CardHeader>
              <CardContent>
                <form>
                  <div class="flex flex-col gap-6">
                    <div class="grid gap-2">
                      <label class="text-sm font-semibold" for="email">{"Email"}</label>
                      <Input
                        id="email"
                        r\#type="email"
                        placeholder="m@example.com"
                      />
                    </div>
                    <div class="grid gap-2">
                      <div class="flex items-center">
                        <label class="text-sm font-semibold" for="password">{"Password"}</label>
                        <a
                          href="/"
                          class={"ml-auto inline-block text-sm text-blue-500 underline-offset-4 hover:underline"}
                        >
                          {"Forgot your password?"}
                        </a>
                      </div>
                      <Input id="password" r#type="password"/>
                    </div>
                  </div>
                </form>
              </CardContent>
              <CardFooter class="flex-col gap-2">
                <Button class={"w-full"}>
                  {"Login"}
                </Button>
                <Button variant={ButtonVariant::Secondary} class={"w-full"}>
                  {"Login with Google"}
                </Button>
              </CardFooter>
            </Card>
          "#}
        >
          <Card>
            <CardHeader>
              <CardTitle>{"Login to your account"}</CardTitle>
              <CardDescription>
                {"Enter your email below to login to your account"}
              </CardDescription>
              <CardAction>
                <Button href={"/card#"} variant={ButtonVariant::Secondary}>{"Sign Up"}</Button>
              </CardAction>
            </CardHeader>
            <CardContent>
              <form>
                <div class="flex flex-col gap-6">
                  <div class="grid gap-2">
                    <label class="text-sm font-semibold" for="email">{"Email"}</label>
                    <Input
                      id="email"
                      r#type="email"
                      placeholder="m@example.com"
                    />
                  </div>
                  <div class="grid gap-2">
                    <div class="flex items-center">
                      <label class="text-sm font-semibold" for="password">{"Password"}</label>
                      <a
                        href="#"
                        class={"ml-auto inline-block text-sm text-blue-500 underline-offset-4 hover:underline"}
                      >
                        {"Forgot your password?"}
                      </a>
                    </div>
                    <Input id="password" r#type="password"/>
                  </div>
                </div>
              </form>
            </CardContent>
            <CardFooter class="flex-col gap-2">
              <Button class={"w-full"}>
                {"Login"}
              </Button>
              <Button variant={ButtonVariant::Secondary} class={"w-full"}>
                {"Login with Google"}
              </Button>
            </CardFooter>
          </Card>
        </PreviewContainer>
      </div>
    </div>
  }
}
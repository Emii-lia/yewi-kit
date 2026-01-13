use yew::{function_component, html, use_state, Callback, Html};
use crate::components::{Checkbox, CheckboxVariant};
use crate::previews::PreviewContainer;
use crate::types::{Color, Size};

#[function_component(CheckboxPreview)]
pub fn checkbox_preview() -> Html {
  let checked = use_state(||true);

  let onchange = {
    let checked = checked.clone();
    Callback::from(move |check: bool| checked.set(check))
  };

  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">{"Checkbox"}</h1>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Display checkboxes with various styles, variants and sizes."}
        </div>
        <pre class="code-block">
          <code>
{"yewi add checkbox"}
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
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_default_1"}
                id={"checkbox_default_1"}
                size={Size::Small} />
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_default_2"}
                id={"checkbox_default_2"}
                size={Size::Medium} />
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_default_3"}
                id={"checkbox_default_3"}
                size={Size::Large} />
            "#}
          >
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_default_1"}
              id={"checkbox_default_1"}
              size={Size::Small} />
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_default_2"}
              id={"checkbox_default_2"}
              size={Size::Medium} />
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_default_3"}
              id={"checkbox_default_3"}
              size={Size::Large} />
          </PreviewContainer>
          <PreviewContainer
            title={"Button"}
            code={r#"
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_button_1"}
                id={"checkbox_button_1"}
                label={"Check"} size={Size::Small}  variant={CheckboxVariant::Button}/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_button_2"}
                id={"checkbox_button_2"}
                label={"Check"} size={Size::Medium}  variant={CheckboxVariant::Button}/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_button_3"}
                id={"checkbox_button_3"}
                label={"Check"} size={Size::Large}  variant={CheckboxVariant::Button}/>
            "#}
          >
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_button_1"}
              id={"checkbox_button_1"}
              label={"Check"} size={Size::Small}  variant={CheckboxVariant::Button}/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_button_2"}
              id={"checkbox_button_2"}
              label={"Check"} size={Size::Medium}  variant={CheckboxVariant::Button}/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_button_3"}
              id={"checkbox_button_3"}
              label={"Check"} size={Size::Large}  variant={CheckboxVariant::Button}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Toggle"}
            code={r#"
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_toggle_1"}
                id={"checkbox_toggle_1"}
                size={Size::Small}  variant={CheckboxVariant::Toggle}/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_toggle_2"}
                id={"checkbox_toggle_2"}
                size={Size::Medium}  variant={CheckboxVariant::Toggle}/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_toggle_3"}
                id={"checkbox_toggle_3"}
                size={Size::Large}  variant={CheckboxVariant::Toggle}/>
            "#}
          >
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_toggle_1"}
              id={"checkbox_toggle_1"}
              size={Size::Small}  variant={CheckboxVariant::Toggle}/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_toggle_2"}
              id={"checkbox_toggle_2"}
              size={Size::Medium}  variant={CheckboxVariant::Toggle}/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_toggle_3"}
              id={"checkbox_toggle_3"}
              size={Size::Large}  variant={CheckboxVariant::Toggle}/>
          </PreviewContainer>
          <PreviewContainer
            title={"Disabled"}
            code={r#"
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_disabled_1"}
                id={"checkbox_disabled_1"}
                variant={CheckboxVariant::Default} disabled=true checked=true/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_disabled_2"}
                id={"checkbox_disabled_2"}
                label={"Check"} variant={CheckboxVariant::Button} disabled=true checked=true/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_disabled_3"}
                id={"checkbox_disabled_3"}
                variant={CheckboxVariant::Toggle} disabled=true checked=true/>
            "#}
          >
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_disabled_1"}
              id={"checkbox_disabled_1"}
              variant={CheckboxVariant::Default} disabled=true checked=true/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_disabled_2"}
              id={"checkbox_disabled_2"}
              label={"Check"} variant={CheckboxVariant::Button} disabled=true checked=true/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_disabled_3"}
              id={"checkbox_disabled_3"}
              variant={CheckboxVariant::Toggle} disabled=true checked=true/>
          </PreviewContainer>
  
          <PreviewContainer
            title={"Colours"}
            code={r#"
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_color_1"}
                id={"checkbox_color_1"}
                variant={CheckboxVariant::Default} color={Color::Rose}/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_color_2"}
                id={"checkbox_color_2"}
                variant={CheckboxVariant::Default} color={Color::Teal}/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_color_3"}
                id={"checkbox_color_3"}
                variant={CheckboxVariant::Button} label={"Check"} color={Color::Rose}/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_color_4"}
                id={"checkbox_color_4"}
                variant={CheckboxVariant::Button} label={"Check"} color={Color::Teal}/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_color_5"}
                id={"checkbox_color_5"}
                variant={CheckboxVariant::Toggle} color={Color::Rose}/>
              <Checkbox
                checked={*checked}
                on_change={onchange.clone()}
                name={"checkbox_color_6"}
                id={"checkbox_color_6"}
                variant={CheckboxVariant::Toggle} color={Color::Teal}/>
            "#}
          >
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_color_1"}
              id={"checkbox_color_1"}
              variant={CheckboxVariant::Default} color={Color::Rose}/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_color_2"}
              id={"checkbox_color_2"}
              variant={CheckboxVariant::Default} color={Color::Teal}/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_color_3"}
              id={"checkbox_color_3"}
              variant={CheckboxVariant::Button} label={"Check"} color={Color::Rose}/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_color_4"}
              id={"checkbox_color_4"}
              variant={CheckboxVariant::Button} label={"Check"} color={Color::Teal}/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_color_5"}
              id={"checkbox_color_5"}
              variant={CheckboxVariant::Toggle} color={Color::Rose}/>
            <Checkbox
              checked={*checked}
              on_change={onchange.clone()}
              name={"checkbox_color_6"}
              id={"checkbox_color_6"}
              variant={CheckboxVariant::Toggle} color={Color::Teal}/>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}
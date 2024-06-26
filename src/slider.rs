use web_sys::{HtmlFormElement as HTMLFormElement, NodeList};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Whether or not the slider is disabled.
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    /// The slider minimum value
    #[prop_or(Some(0))]
    pub min: Option<usize>,
    /// The slider maximum value
    #[prop_or(Some(100))]
    pub max: Option<usize>,
    /// The slider value displayed when range is false.
    #[prop_or(None)]
    pub value: Option<usize>,
    /// The slider start value displayed when range is true.
    #[prop_or(None)]
    pub value_start: Option<usize>,
    /// The slider end value displayed when range is true.
    #[prop_or(None)]
    pub value_end: Option<usize>,
    /// An optional label for the slider’s value displayed when range is false; if not set, the
    /// label is the value itself.
    #[prop_or(Some(AttrValue::Static("")))]
    pub value_label: Option<AttrValue>,
    /// An optional label for the slider’s start value displayed when range is true; if not set,
    /// the label is the valueStart itself.
    #[prop_or(Some(AttrValue::Static("")))]
    pub value_label_start: Option<AttrValue>,
    /// An optional label for the slider’s end value displayed when range is true; if not set, the
    /// label is the valueEnd itself.
    #[prop_or(Some(AttrValue::Static("")))]
    pub value_label_end: Option<AttrValue>,
    /// Aria label for the slider’s start handle displayed when range is true.
    #[prop_or(Some(AttrValue::Static("")))]
    pub aria_label_start: Option<AttrValue>,
    /// Aria value text for the slider’s start value displayed when range is true.
    #[prop_or(Some(AttrValue::Static("")))]
    pub aria_value_text_start: Option<AttrValue>,
    /// Aria label for the slider’s end handle displayed when range is true.
    #[prop_or(Some(AttrValue::Static("")))]
    pub aria_label_end: Option<AttrValue>,
    /// Aria value text for the slider’s end value displayed when range is true.
    #[prop_or(Some(AttrValue::Static("")))]
    pub aria_value_text_end: Option<AttrValue>,
    /// The step between values.
    #[prop_or(Some(1))]
    pub step: Option<usize>,
    /// Whether or not to show tick marks.
    #[prop_or(Some(false))]
    pub ticks: Option<bool>,
    /// Whether or not to show a value label when activated.
    #[prop_or(Some(false))]
    pub labeled: Option<bool>,
    /// Whether or not to show a value range. When false, the slider displays a slideable handle
    /// for the value property; when true, it displays slideable handles for the valueStart and
    /// valueEnd properties.
    #[prop_or(Some(false))]
    pub range: Option<bool>,
    ///
    #[prop_or(None)]
    pub name: Option<AttrValue>,
    ///
    #[prop_or(None)]
    pub name_start: Option<AttrValue>,
    ///
    #[prop_or(None)]
    pub name_end: Option<AttrValue>,
    ///
    #[prop_or(None)]
    pub form: Option<HTMLFormElement>,
    ///
    #[prop_or(None)]
    pub labels: Option<NodeList>,
}

#[function_component]
pub fn Slider(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/slider.js")
    });
    html! { <md-slider
        disabled={props.disabled.unwrap_or_default()}
        ~min={crate::js_value_or_null(props.min.clone())}
        ~max={crate::js_value_or_null(props.max.clone())}
        // TODO: ~value={props.value.clone().unwrap_or_default()}
        ~valueStart={crate::js_value_or_null(props.value_start.clone())}
        ~valueEnd={crate::js_value_or_null(props.value_end.clone())}
        ~valueLabel={crate::js_value_from_string_or_null(props.value_label.as_ref())}
        ~valueLabelStart={crate::js_value_from_string_or_null(props.value_label_start.as_ref())}
        ~valueLabelEnd={crate::js_value_from_string_or_null(props.value_label_end.as_ref())}
        ~ariaLabelStart={crate::js_value_from_string_or_null(props.aria_label_start.as_ref())}
        ~ariaValueTextStart={crate::js_value_from_string_or_null(props.aria_value_text_start.as_ref())}
        ~ariaLabelEnd={crate::js_value_from_string_or_null(props.aria_label_end.as_ref())}
        ~ariaValueTextEnd={crate::js_value_from_string_or_null(props.aria_value_text_end.as_ref())}
        ~step={crate::js_value_or_null(props.step.clone())}
        ~ticks={crate::js_value_or_null(props.ticks.clone())}
        ~labeled={crate::js_value_or_null(props.labeled.clone())}
        ~range={crate::js_value_or_null(props.range.clone())}
        ~name={crate::js_value_from_string_or_null(props.name.as_ref())}
        ~nameStart={crate::js_value_from_string_or_null(props.name_start.as_ref())}
        ~nameEnd={crate::js_value_from_string_or_null(props.name_end.as_ref())}
        ~form={crate::js_value_or_null(props.form.clone())}
        ~labels={crate::js_value_or_null(props.labels.clone())}
    />}
}

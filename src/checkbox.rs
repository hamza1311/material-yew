use wasm_bindgen::JsValue;
use yew::prelude::*;
use web_sys::HtmlFormElement as HTMLFormElement;
use web_sys::NodeList;

type ValidityState = JsValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[doc = "Whether or not the checkbox is selected."]
    #[prop_or(Some(false))]
    pub checked: Option<bool>,
    #[doc = "Whether or not the checkbox is disabled."]
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    #[doc = "Whether or not the checkbox is indeterminate.<br>https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox#indeterminate_state_checkboxes"]
    #[prop_or(Some(false))]
    pub indeterminate: Option<bool>,
    #[doc = "When true, require the checkbox to be selected when participating in form submission.<br>https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox#validation"]
    #[prop_or(Some(false))]
    pub required: Option<bool>,
    #[doc = "The value of the checkbox that is submitted with a form when selected.<br>https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox#value"]
    #[prop_or(Some(AttrValue::Static("on")))]
    pub value: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(None)]
    pub name: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(None)]
    pub form: Option<HTMLFormElement>,
    #[doc = ""]
    #[prop_or(None)]
    pub labels: Option<NodeList>,
    #[doc = ""]
    #[prop_or(None)]
    pub validitype: Option<ValidityState>,
    #[doc = ""]
    #[prop_or(None)]
    pub validation_message: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(None)]
    pub will_validate: Option<bool>,
    pub children: Html,
}

#[function_component]
pub fn Checkbox(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/checkbox.js")
    });
    html! { <md-checkbox
        ~checked={crate::js_value_or_null(props.checked.clone())}
        disabled={props.disabled.unwrap_or_default()}
        ~indeterminate={crate::js_value_or_null(props.indeterminate.clone())}
        required={props.required.unwrap_or_default()}
        value={props.value.clone().unwrap_or_default()}
        ~name={crate::js_value_from_string_or_null(props.name.as_ref())}
        ~form={crate::js_value_or_null(props.form.clone())}
        ~labels={crate::js_value_or_null(props.labels.clone())}
        ~validity={crate::js_value_or_null(props.validitype.clone())}
        ~validationMessage={crate::js_value_from_string_or_null(props.validation_message.as_ref())}
        ~willValidate={crate::js_value_or_null(props.will_validate.clone())}
    > {props.children.clone()} </md-checkbox> }
}

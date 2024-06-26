use web_sys::{HtmlFormElement as HTMLFormElement, NodeList};
use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    /// Whether or not the radio is disabled.
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    /// The element value to use in form submission when checked.
    #[prop_or(Some(AttrValue::Static("on")))]
    pub value: Option<AttrValue>,
    ///
    #[prop_or(None)]
    pub checked: Option<bool>,
    ///
    #[prop_or(None)]
    pub name: Option<AttrValue>,
    ///
    #[prop_or(None)]
    pub form: Option<HTMLFormElement>,
    ///
    #[prop_or(None)]
    pub labels: Option<NodeList>,
    #[prop_or(None)]
    pub oninput: Option<Callback<InputEvent>>,
    #[prop_or(None)]
    pub onchange: Option<Callback<Event>>,
}

#[function_component]
pub fn Radio(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/radio.js")
    });
    html! { <md-radio
        disabled={props.disabled.unwrap_or_default()}
        value={props.value.clone().unwrap_or_default()}
        ~checked={crate::js_value_or_null(props.checked.clone())}
        ~name={crate::js_value_from_string_or_null(props.name.as_ref())}
        ~form={crate::js_value_or_null(props.form.clone())}
        ~labels={crate::js_value_or_null(props.labels.clone())}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
    /> }
}

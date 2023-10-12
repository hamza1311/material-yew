//!
//!
//! ## TODOs
//!
//! ### Missing Components
//! - [`md-chip-set`](https://material-web.dev/components/chip/#mdchipset-lessmd-chip-setgreater)
//! - [`dialog`](https://material-web.dev/components/dialog/)
mod button;
mod radio;
mod menu_item;
mod sub_menu;
mod menu;

pub use button::{Button, ButtonVariants};
pub use radio::{Radio};
pub use menu_item::{MenuItem};
pub use sub_menu::{SubMenu};
pub use menu::{Menu};

use wasm_bindgen::prelude::*;
use yew::AttrValue;

macro_rules! import_material_web_module {
    ($module:literal) => {{
        #[wasm_bindgen::prelude::wasm_bindgen(module = $module)]
        extern "C" {
            #[wasm_bindgen(getter)]
            fn __dummy_loader() -> wasm_bindgen::JsValue;
        }

        #[allow(dead_code)]
        static LOADED: std::sync::Once = std::sync::Once::new();
        {
            LOADED.call_once(|| {
                __dummy_loader();
            });
        }
    }};
}

pub(crate) use import_material_web_module;

pub(crate) fn load_core() {
    import_material_web_module!("/md-web/core.js");
}

fn js_value_or_null<T>(v: Option<T>) -> JsValue
where
    JsValue: From<T>,
{
    match v {
        Some(v) => JsValue::from(v),
        None => JsValue::NULL,
    }
}

fn js_value_from_string_or_null(v: Option<&AttrValue>) -> Option<JsValue> {
    match v {
        Some(v) => Some(JsValue::from_str(&*v)),
        None => None,
    }
}

use js_sys::{Function, Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "countly-sdk-web")]
extern "C" {
    #[wasm_bindgen(js_name = default)]
    pub type Countly;

    #[wasm_bindgen(static_method_of = Countly, js_class = "default")]
    pub fn init(config: JsValue);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default", getter = q)]
    pub fn queue() -> Array;

    #[wasm_bindgen(static_method_of = Countly, js_class = "default", setter = getViewName)]
    pub fn set_view_name_getter(fun: &Function);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default", setter = getViewUrl)]
    pub fn set_view_url_getter(fun: &Function);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default")]
    pub fn collect_from_facebook(custom_properties: JsValue);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default")]
    pub fn group_features(groups: JsValue);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default")]
    pub fn add_consent(features: Array);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default")]
    pub fn remove_consent(features: Array);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default", setter = remote_config)]
    pub fn set_remote_config(callback: &Function);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default")]
    pub fn get_remote_config() -> JsValue;

    #[wasm_bindgen(static_method_of = Countly, js_class = "default", js_name = get_remote_config)]
    pub fn get_remote_config_for_key(key: &str) -> JsValue;

    #[wasm_bindgen(static_method_of = Countly, js_class = "default")]
    pub fn fetch_remote_config(callback: &Function);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default", js_name = fetch_remote_config)]
    pub fn fetch_remote_config_for_keys(keys: Array, callback: &Function);

    #[wasm_bindgen(static_method_of = Countly, js_class = "default", js_name = fetch_remote_config)]
    pub fn fetch_remote_config_except_for_keys(null: JsValue, keys: Array, callback: &Function);

}

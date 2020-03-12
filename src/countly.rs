use std::collections::HashMap;
use crate::{
    Config,
    countly_sys::Countly as CountlySys,
    gdpr::ConsentFeatures,
};
use wasm_bindgen::{JsValue, JsCast, closure::Closure};
use js_sys::Array;
use serde::Serialize;

pub struct Countly;

impl Countly {
    /// Call this function before anything else.
    pub fn configure(config: Config) {
        CountlySys::init(JsValue::from_serde(&config).unwrap());
    }

    fn queue() -> Array {
        CountlySys::queue()
    }

    /// This method will automatically track user sessions, by calling begin extend and end session methods.
    pub fn enable_session_tracking() {
        Self::queue().push(Array::of1(&JsValue::from_str("track_sessions")).unchecked_ref());
    }

    /// This method will track current pageview, by using `location.path` as page name and report it to server.
    pub fn track_pageview() {
        Self::queue().push(Array::of1(&JsValue::from_str("track_pageview")).unchecked_ref());
    }

    /// For Ajax updated contents and single page web applications, pass page name as a parameter to record new page view.
    pub fn track_pageview_with_name(name: &str) {
        Self::queue().push(Array::of2(&JsValue::from_str("track_pageview"), &JsValue::from_str(name)).unchecked_ref());
    }

    /// In some cases you want to ignore some URLs to exclude from tracking, like dynamic URLs including user id in the
    /// URL, or internal URLs, or any other reason. You can do so, by providing another parameter with list of strings
    /// of views to ignore or list of regular expressions to ignore.
    pub fn track_pageview_with_filter(filter: &[&str]) {
        Self::queue().push(Array::of2(&JsValue::from_str("track_pageview"), &JsValue::from_serde(&filter).unwrap()).unchecked_ref());
    }

    /// In some cases you want to ignore some URLs to exclude from tracking, like dynamic URLs including user id in the
    /// URL, or internal URLs, or any other reason. You can do so, by providing another parameter with list of strings
    /// of views to ignore or list of regular expressions to ignore.
    pub fn track_pageview_with_name_and_filter(name: &str, filter: &[&str]) {
        Self::queue().push(Array::of3(&JsValue::from_str("track_pageview"), &JsValue::from_str(name), &JsValue::from_serde(&filter).unwrap()).unchecked_ref());
    }

    /// There are cases when there is a more complex logic to determining view name and in some cases you need to separate
    /// URL and View naming. So you still have some business logic view names, but you have valid URL underneath them to
    /// view action maps, like clicks and scrolls.
    /// 
    /// Note that this leaks the closure passed. Since you should only set this once and it should last for the whole session, this should be fine.
    pub fn set_view_name_callback(callback: impl FnMut() -> String + 'static) {
        let wrapper = Closure::wrap(Box::new(callback) as Box<dyn FnMut() -> String>);
        CountlySys::set_view_name_getter(wrapper.as_ref().unchecked_ref());
        wrapper.forget();
    }

    /// There are cases when there is a more complex logic to determining view name and in some cases you need to separate
    /// URL and View naming. So you still have some business logic view names, but you have valid URL underneath them to
    /// view action maps, like clicks and scrolls.
    /// 
    /// Note that this leaks the closure passed. Since you should only set this once and it should last for the whole session, this should be fine.
    pub fn set_view_url_callback(callback: impl FnMut() -> String + 'static) {
        let wrapper = Closure::wrap(Box::new(callback) as Box<dyn FnMut() -> String>);
        CountlySys::set_view_url_getter(wrapper.as_ref().unchecked_ref());
        wrapper.forget();
    }

    /// This method will track click to specific links and will report with custom events with key linkClick and link's text, id and url as segments.
    ///
    /// By default all links would be tracked for whole page, but you may provide the parent node as a parameter for which to track link clicks.
    pub fn enable_link_tracking(parent: Option<web_sys::Element>) {
        if let Some(parent) = parent {
            Self::queue().push(Array::of2(&JsValue::from_str("track_links"), parent.unchecked_ref()).unchecked_ref());
        } else {
            Self::queue().push(Array::of1(&JsValue::from_str("track_links")).unchecked_ref());
        }
    }

    /// This method will automatically track form submissions and collect form data and input values in the form and report as Custom Event with formSubmit key
    ///
    /// By default all forms would be tracked for whole page, but you may provide the parent node as a parameter for which to track forms.
    ///
    /// The second parameter controls whether to collect hidden inputs or not. By default hidden inputs are not collected.
    pub fn enable_form_submission_tracking(parent: Option<&web_sys::Element>, include_hidden: bool) {
        let null = JsValue::NULL;
        let parent = if let Some(parent) = parent {
            parent.unchecked_ref()
        } else {
            &null
        };
        Self::queue().push(Array::of3(&JsValue::from_str("track_forms"), parent, &JsValue::from_bool(include_hidden)).unchecked_ref());
    }

    /// When using Countly attribution analytics, you can also report conversion to Countly server, like for example when visitor purchased something
    /// or registered.
    ///
    /// By default if user came to your website through Countly campaign link, campaign information will be automatically stored for this users and
    /// used when reporting conversion. If conversion is not reported yet, then when visiting through other campaign link, campaign information will
    /// be overwritten, so when you report the conversion, it would report the latest campaign user used to access your website.
    ///
    /// But you can also overwrite that data and provide some specific campaign id for which you want to report conversion.
    ///
    /// If there is no stored campaign data and you don't provide any campaign id, then conversion will not be reported.
    ///
    /// Note: that conversion for each user may be reported only once, all other conversions will be ignored for this same user.
    pub fn enable_conversion_reporting(name: Option<&str>) {
        if let Some(name) = name {
            Self::queue().push(Array::of2(&JsValue::from_str("report_conversion"), &JsValue::from_str(name)).unchecked_ref());
        } else {
            Self::queue().push(Array::of1(&JsValue::from_str("report_conversion")).unchecked_ref());
        }
    }
    
    /// Resume tracking after a call to [Countly::opt_out]
    /// 
    /// If you want to achieve opt out by default state, combine these methods with initial setting ignore_visitor on Countly init object.
    pub fn opt_in() {
        Self::queue().push(Array::of1(&JsValue::from_str("opt_in")).unchecked_ref());
    }
    
    /// By default Countly SDK is always opt in, but you can easily disable all tracking by calling opt_out method.
    /// It will also persistently save setting and prevent tracking after page reloads too.
    /// 
    /// If you want to achieve opt out by default state, combine these methods with initial setting ignore_visitor on Countly init object.
    pub fn opt_out() {
        Self::queue().push(Array::of1(&JsValue::from_str("opt_out")).unchecked_ref());
    }

    /// This method will look into forms filled by your users and will try to gather data like name, email address, username, etc from
    /// that data.
    /// 
    /// By default all forms will be checked, but optionally you can provide form element if you want to collect data only from specific
    /// form, or call method multiple times for different forms. Also if you already provide data for users, you would not want to over
    /// write it, so you can provide second parameter as true to indicate that found data should be stored in custom properties.
    pub fn enable_form_data_collection(parent: Option<&web_sys::Element>, custom_properties: bool) {
        let null = JsValue::NULL;
        let parent = if let Some(parent) = parent {
            parent.unchecked_ref()
        } else {
            &null
        };
        Self::queue().push(Array::of3(&JsValue::from_str("collect_from_forms"), parent, &JsValue::from_bool(custom_properties)).unchecked_ref());
    }

    /// If your website uses Facebook Javascript SDK, you can use this helper method to automatically collect user data from their
    /// Facebook account. Just call the method right after Facebook SDK initialization and optionally provide object with custom
    /// properties and graph paths for values where to get them.
    pub fn collect_from_facebook(custom_properties: &HashMap<String, String>) {
        CountlySys::collect_from_facebook(JsValue::from_serde(&custom_properties).unwrap());
    }

    /// Custom event is a way to track any custom actions or other data you want to track from your website. You can also provide
    /// segments to be able to view breakdown of action by provided segment values.
    /// 
    /// Custom event consists of Javascript object with keys:
    /// 
    /// * key - the name of the event
    /// * count - number of events
    /// * sum - sum to report with event
    /// * dur - duration in seconds to report with event
    /// * segmentation - an object with key/value pairs to report with event as segments
    pub fn add_event(key: &str, count: u32, sum: Option<u32>, duration: Option<f64>, segmentation: HashMap<String, String>) {
        Self::queue().push(Array::of2(&JsValue::from_str("add_event"), &JsValue::from_serde(&CustomEvent {
            key: key.to_owned(),
            count, sum, duration, segmentation,
        }).unwrap()).unchecked_ref());
    }

    /// You can report time or duration with every event by providing dur property of the events object. But if you want, you can
    /// also let Web SDK to track duration of some specific event for you, you can use [Countly::start_event] and [Countly::end_event] methods.
    pub fn start_event(name: &str) {
        Self::queue().push(Array::of2(&JsValue::from_str("start_event"), &JsValue::from_str(name)).unchecked_ref());
    }
    
    /// Countly will internally mark the start of event and will wait until you end event with end_event method, setting up
    /// dur property based on how much time has passed since start_event for same event name was called.
    pub fn end_event(name: &str) {
        Self::queue().push(Array::of2(&JsValue::from_str("end_event"), &JsValue::from_str(name)).unchecked_ref());
    }
    
    /// If you have any details about the user/visitor, you can provide Countly with that information. This will allow you
    /// track each and specific user on "User Profiles" tab, which is available with Countly Enterprise Edition.
    pub fn set_user_details(details: UserDetails) {
        Self::queue().push(Array::of2(&JsValue::from_str("user_details"), &JsValue::from_serde(&details).unwrap()).unchecked_ref());
    }
    
    /// Set custom property.
    pub fn user_data_set(key: &str, value: Value) {
        Self::queue().push(Array::of3(&JsValue::from_str("userData.set"), &JsValue::from_str(&key), &value.into()).unchecked_ref());
    }

    /// Remove custom property.
    pub fn user_data_unset(key: &str) {
        Self::queue().push(Array::of2(&JsValue::from_str("userData.unset"), &JsValue::from_str(&key)).unchecked_ref());
    }
    
    /// Set custom property only if property does not exist.
    pub fn user_data_set_once(key: &str, value: Value) {
        Self::queue().push(Array::of3(&JsValue::from_str("userData.set_once"), &JsValue::from_str(&key), &value.into()).unchecked_ref());
    }
    
    /// Increment value in key by one.
    pub fn user_data_increment(key: &str) {
        Self::queue().push(Array::of2(&JsValue::from_str("userData.increment"), &JsValue::from_str(&key)).unchecked_ref());
    }
    
    /// Increment value in key by provided value.
    pub fn user_data_increment_by(key: &str, value: f64) {
        Self::queue().push(Array::of3(&JsValue::from_str("userData.increment_by"), &JsValue::from_str(&key), &JsValue::from_f64(value)).unchecked_ref());
    }
    
    /// Multiply value in key by provided value.
    pub fn user_data_multiply(key: &str, value: f64) {
        Self::queue().push(Array::of3(&JsValue::from_str("userData.multiply"), &JsValue::from_str(&key), &JsValue::from_f64(value)).unchecked_ref());
    }
    
    /// Save max value between current and provided.
    pub fn user_data_max(key: &str, value: f64) {
        Self::queue().push(Array::of3(&JsValue::from_str("userData.max"), &JsValue::from_str(&key), &JsValue::from_f64(value)).unchecked_ref());
    }
    
    /// Save min value between current and provided.
    pub fn user_data_min(key: &str, value: f64) {
        Self::queue().push(Array::of3(&JsValue::from_str("userData.min"), &JsValue::from_str(&key), &JsValue::from_f64(value)).unchecked_ref());
    }
    
    /// Add value to key as array element.
    pub fn user_data_push(key: &str, value: Value) {
        Self::queue().push(Array::of3(&JsValue::from_str("userData.push"), &JsValue::from_str(&key), &value.into()).unchecked_ref());
    }
    
    /// Add value to key as array element, but only store unique values in array.
    pub fn user_data_push_unique(key: &str, value: Value) {
        Self::queue().push(Array::of3(&JsValue::from_str("userData.push_unique"), &JsValue::from_str(&key), &value.into()).unchecked_ref());
    }
    
    /// Remove value from array under property with key as name
    pub fn user_data_pull(key: &str, value: Value) {
        Self::queue().push(Array::of3(&JsValue::from_str("userData.pull"), &JsValue::from_str(&key), &value.into()).unchecked_ref());
    }

    /// Send userData to server.
    pub fn user_data_save() {
        Self::queue().push(Array::of1(&JsValue::from_str("userData.save")).unchecked_ref());
    }

    /// To automatically capture and report Javascript errors on your website, call this function.
    /// 
    /// You can additionally add more segments or properties/values to track with error reports, by providing an object with
    /// key/values to add to error reports.
    pub fn enable_track_errors(segments: Option<HashMap<String, String>>) {
        if let Some(segments) = segments {
            Self::queue().push(Array::of2(&JsValue::from_str("track_errors"), &JsValue::from_serde(&segments).unwrap()).unchecked_ref());
        } else {
            Self::queue().push(Array::of1(&JsValue::from_str("track_errors")).unchecked_ref());
        }
    }

    /// Apart from reporting unhandled errors automatically, you can also report handled exceptions to server too, so you can figure
    /// out how and even if you need to handle them later on. And optionally you can again provide custom segments to be used in the
    /// report (or use the ones provided with track_error method as default ones).
    pub fn log_error(error: JsValue, segments: Option<HashMap<String, String>>) {
        if let Some(segments) = segments {
            Self::queue().push(Array::of3(&JsValue::from_str("log_error"), &error, &JsValue::from_serde(&segments).unwrap()).unchecked_ref());
        } else {
            Self::queue().push(Array::of2(&JsValue::from_str("log_error"), &error).unchecked_ref());
        }
    }
    
    /// To better understand what your users did prior to getting an error, you can leave out breadcrumbs through out the code,
    /// on different user actions. This breadcrumb will be then combined in single log and reported to server too.
    pub fn add_log(msg: &str) {
        Self::queue().push(Array::of2(&JsValue::from_str("add_log"), &JsValue::from_str(msg)).unchecked_ref());
    }

    /// In some cases you may want to change the ID of the user/device that you provided or Countly generated automatically,
    /// for example, when user was changed.
    /// 
    /// In some cases, you may also need to change user's device ID in a way, that server will merge data of both user IDs
    /// (existing and new ID you provided) on the server, eg when user used website without authenticating and have recorded
    /// some data, and then authenticated and you want to change ID to your internal id of this user, to keep tracking it across
    /// multiple devices. To enable this, set `merge` to `true`.
    pub fn change_device_id(id: &str, merge: bool) {
        Self::queue().push(Array::of3(&JsValue::from_str("change_id"), &JsValue::from_str(id), &JsValue::from_bool(merge)).unchecked_ref());
    }
    
    /// Depending on your website and use case, you may want to combine some of the consent features into one.
    /// 
    /// After this call [Countly::add_consent] to allow this specific combination of features.
    pub fn group_features(groups: HashMap<String, Vec<ConsentFeatures>>) {
        CountlySys::group_features(JsValue::from_serde(&groups).unwrap());
    }

    /// Upon visitor arriving at your website, you should check if you already have consent from this visitor. If not, you should
    /// present him with a popup explaining what will be tracked and allowing to consent to tracking. When user selected consent
    /// preferences, you should persistently store it and on each Countly load, let Countly know which features did user consent
    /// to by calling this method and passing one or multiple features.
    pub fn add_consent(features: &[&str]) {
        CountlySys::add_consent(JsValue::from_serde(features).unwrap().unchecked_into());
    }

    /// You should also allow user to change their mind in, for example, separate settings screen and upon changes made there,
    /// call respective Countly.add_consent or Countly.remove_consent methods, to let Countly track specific features or disable
    /// tracking for them.
    pub fn remove_consent(features: &[&str]) {
        CountlySys::remove_consent(JsValue::from_serde(features).unwrap().unchecked_into());
    }

    /// This method would allow you to control sessions manually. Use it only, if you don't call track_sessions method and set
    /// `use_session_cookie` setting to false, for more granular control of the session.
    /// 
    /// If `no_heart_beat` is `true`, then Countly WebSDK won't extend session automatically, and you would need to do that automatically.
    pub fn begin_session(no_heart_beat: bool) {
        if no_heart_beat {
            Self::queue().push(Array::of2(&JsValue::from_str("begin_session"), &JsValue::TRUE).unchecked_ref());
        } else {
            Self::queue().push(Array::of1(&JsValue::from_str("begin_session")).unchecked_ref());
        }
    }

    /// By default (if `no_heart_beat` was false in [Countly::begin_session]) Countly SDK will extend session itself, but if you chose not
    /// to, then you can extend is using this method and provide seconds since last call [Countly::begin_session] or [Countly::extend_session] call,
    /// whatever was the last one.
    pub fn extend_session(secs: f64) {
        Self::queue().push(Array::of2(&JsValue::from_str("session_duration"), &JsValue::from_f64(secs)).unchecked_ref());
    }

    /// When visitor is leaving your app or website, you should end his session with this method, optionally providing amount of
    /// seconds since last [Countly::begin_session] or [Countly::extend_session] calls, whatever was the last one.
    pub fn end_session(secs: Option<f64>) {
        if let Some(secs) = secs {
            Self::queue().push(Array::of2(&JsValue::from_str("end_session"), &JsValue::from_f64(secs)).unchecked_ref());
        } else {
            Self::queue().push(Array::of1(&JsValue::from_str("end_session")).unchecked_ref());
        }
    }

    /// There are cases, when you want SDK to collect data, but not send it to the server until certain point. Additionally, it
    /// allows to delay providing `device_id` property until later time.
    ///
    /// For example, if you want to track your users with custom `device_id`, like your, internal customer id. And you can only
    /// get that value, when user logged in. But you also want to track, what user did before logging in.
    ///
    /// Using offline mode in such context, allows you to omit user merging and server overhead that comes with it, including
    /// possibly skewed aggregation data.
    ///
    /// To start SDK in offline mode, simply provide `offline_mode` config value as `true`. And at this point, you can omit
    /// providing `device_id` value, if you want.
    /// 
    /// Or you can enable offline at any point later in SDK with this function.
    pub fn enable_offline_mode() {
        Self::queue().push(Array::of1(&JsValue::from_str("enable_offline_mode")).unchecked_ref());
    }

    /// When you want to disable offline mode and optionally provide `device_id`, you can do it with this function.
    pub fn disable_offline_mode(device_id: Option<&str>) {
        if let Some(device_id) = device_id {
            Self::queue().push(Array::of2(&JsValue::from_str("disable_offline_mode"), &JsValue::from_str(device_id)).unchecked_ref());
        } else {
            Self::queue().push(Array::of1(&JsValue::from_str("disable_offline_mode")).unchecked_ref());
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct CustomEvent {
    key: String,
    count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    sum: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,
    segmentation: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct UserDetails {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub username: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub email: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub organization: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub phone: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub picture: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub gender: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byear: Option<u32>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub custom: HashMap<String, String>,
}

pub enum Value {
    Text(String),
    Number(f64),
    Array(Vec<Value>),
}

impl Into<JsValue> for Value {
    fn into(self) -> JsValue {
        match self {
            Self::Text(s) => JsValue::from_str(&s),
            Self::Number(n) => JsValue::from_f64(n),
            Self::Array(v) => {
                let arr = Array::new_with_length(v.len() as _);
                for (idx, element) in v.into_iter().enumerate() {
                    arr.set(idx as _, element.into());
                }
                arr.unchecked_into()
            }
        }
    }
}

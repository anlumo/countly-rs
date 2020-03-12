use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    /// mandatory, app key for your app created in Countly
    app_key: String,
    /// your Countly server url - you can also use your own server URL or IP here
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// to identify a visitor, will be auto generated if not provided
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// the version of your app or website
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// country code for your visitor
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// name of the city of your visitor
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ip address of your visitor
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    /// output debug info into console (default: false)
    pub debug: bool,
    #[serde(skip_serializing_if = "Clone::clone")]
    /// option to ignore traffic from bots (default: true)
    pub ignore_bots: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// set an interval how often to check if there is any data to report and report it (default: 500 ms)
    pub interval: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// maximum amount of queued requests to store (default: 1000)
    pub queue_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// set time in seconds to wait after failed connection to server (default: 60 seconds)
    pub fail_timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// after how many minutes user should be counted as inactive, if he did not perform any actions, as mouse move, scroll or keypress (default: 20 minutes)
    pub inactivity_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// how often in seconds should session be extended (default: 60 seconds)
    pub session_update: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// maximum amount of events to send in one batch (default: 10)
    pub max_events: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// maximum amount of breadcrumbs to store for crash logs (default: 100)
    pub max_logs: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// array with referrers to ignore (default: none)
    pub ignore_referrers: Vec<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    /// ignore prefetching and pre rendering from counting as real website visits (default: true)
    pub ignore_prefetch: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    /// force using post method for all requests (default: false)
    pub force_post: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    /// ignore this current visitor (default: false)
    pub ignore_visitor: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    /// Set to true if you are implementing GDPR compatible consent management. It would prevent running any functionality without proper consent (default: false)
    pub require_consent: bool,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    /// Object instructing which UTM parameters to track (default: `{"source":true, "medium":true, "campaign":true, "term":true, "content":true}`)
    pub utm: HashMap<String, bool>,
    #[serde(skip_serializing_if = "Clone::clone")]
    /// Use cookie to track session (default: true)
    pub use_session_cookie: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// How long till cookie session should expire in minutes (default: 30 minutes)
    pub session_cookie_timeout: Option<f64>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    /// Enable automatic remote config fetching, provide callback function to be notified when fetching done (default: false)
    pub remote_config: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    /// There are cases, when you want SDK to collect data, but not send it to the server until certain point. Additionally,
    /// it allows to delay providing device_id property until later time.
    ///
    /// For example, if you want to track your users with custom device_id, like your, internal customer id. And you can only
    /// get that value, when user logged in. But you also want to track, what user did before logging in.
    ///
    /// Using offline mode in such context, allows you to omit user merging and server overhead that comes with it, including
    /// possibly skewed aggregation data.
    pub offline_mode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sometimes you want to track different parts of the same domain/website as different separate applications.
    ///
    /// If you just instantiate Countly SDK in both places, but with different app ids, then the persistent storage for them
    /// will clash, storing information for both apps. Sometimes it is exactly what you want, as in overall it will work,
    /// they will share device_id and although storing requests together, each will be sent to different app on the same server.
    ///
    /// But there are cases, when you want to keep them completely separate, and for that, you need to provide namespace for
    /// different trackers, so their local storages would not clash.
    pub namespace: Option<String>,
}

impl Config {
    pub fn new(app_key: &str, url: &str) -> Self {
        Config {
            app_key: app_key.to_owned(),
            url: url.to_owned(),
            device_id: None,
            app_version: None,
            country_code: None,
            city: None,
            ip_address: None,
            debug: false,
            ignore_bots: true,
            interval: None,
            queue_size: None,
            fail_timeout: None,
            inactivity_time: None,
            session_update: None,
            max_events: None,
            max_logs: None,
            ignore_referrers: Vec::new(),
            ignore_prefetch: true,
            force_post: false,
            ignore_visitor: false,
            require_consent: false,
            utm: HashMap::new(),
            use_session_cookie: true,
            session_cookie_timeout: None,
            remote_config: false,
            offline_mode: false,
            namespace: None,
        }
    }
}

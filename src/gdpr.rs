#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// SDK provides different features for consent, you check all the supported features for current SDK by checking `Countly.features` property.
pub enum ConsentFeatures {
    /// tracking when, how often and how long users use your website
    Sessions,
    /// allow sending your custom events to server
    Events,
    /// allow tracking which views/pages does user access
    Views,
    /// allow tracking user scrolls for scroll heatmap
    Scrolls,
    /// allow tracking user clicks for heatmap as well as link clicks
    Clicks,
    /// allow tracking user's form submissions
    Forms,
    /// allow tracking javascript errors
    Crashes,
    /// allow tracking from which campaign did user come
    Attribution,
    /// allow collecting/providing user information, including custom properties
    Users,
    /// allowing to rate and provide feedback
    StarRating,
    /// allowing to record location of user (country, city level)
    Location,
}

impl Into<&'static str> for ConsentFeatures {
    fn into(self) -> &'static str {
        match self {
            Self::Sessions => "sessions",
            Self::Events => "events",
            Self::Views => "views",
            Self::Scrolls => "scrolls",
            Self::Clicks => "clicks",
            Self::Forms => "forms",
            Self::Crashes => "crashes",
            Self::Attribution => "attribution",
            Self::Users => "users",
            Self::StarRating => "star-rating",
            Self::Location => "location",
        }
    }
}

impl serde::Serialize for ConsentFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str((*self).into())
    }
}
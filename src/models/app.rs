//! App Store and Customer Review model types for the App Store Connect API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::Resource;

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

/// An App Store Connect app resource.
pub type App = Resource<AppAttributes>;

/// Attributes of an App Store Connect app.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppAttributes {
    /// Display name of the app.
    pub name: Option<String>,
    /// Bundle identifier (e.g., "com.example.myapp").
    pub bundle_id: Option<String>,
    /// SKU assigned to the app.
    pub sku: Option<String>,
    /// Primary locale (e.g., "en-US").
    pub primary_locale: Option<String>,
}

// ---------------------------------------------------------------------------
// Customer Reviews
// ---------------------------------------------------------------------------

/// A customer review resource for an app.
pub type CustomerReview = Resource<CustomerReviewAttributes>;

/// Attributes of a customer review.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerReviewAttributes {
    /// Star rating (1-5).
    pub rating: Option<i32>,
    /// Review title.
    pub title: Option<String>,
    /// Review body text.
    pub body: Option<String>,
    /// Nickname of the reviewer.
    pub reviewer_nickname: Option<String>,
    /// Territory / country code.
    pub territory: Option<String>,
    /// Date the review was created.
    pub created_date: Option<DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// Customer Review Response
// ---------------------------------------------------------------------------

/// A developer response to a customer review.
pub type CustomerReviewResponse = Resource<CustomerReviewResponseAttributes>;

/// Attributes of a customer review response.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerReviewResponseAttributes {
    /// The response body text.
    pub response_body: Option<String>,
    /// Date the response was last modified.
    pub last_modified_date: Option<DateTime<Utc>>,
    /// State of the response (e.g., "PUBLISHED", "PENDING_PUBLISH").
    pub state: Option<String>,
}

#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;

//! Sales report model types and TSV parsing for the App Store Connect API.

use std::io::Read;

use serde::{Deserialize, Serialize};

/// A single row from an App Store Connect sales report.
///
/// Parsed from gzip-compressed TSV data returned by `GET /v1/salesReports`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SalesReportRow {
    /// Content provider name.
    #[serde(rename = "Provider")]
    pub provider: String,
    /// Provider country code.
    #[serde(rename = "Provider Country")]
    pub provider_country: String,
    /// Stock-keeping unit.
    #[serde(rename = "SKU")]
    pub sku: String,
    /// Developer name.
    #[serde(rename = "Developer")]
    pub developer: String,
    /// App or IAP title.
    #[serde(rename = "Title")]
    pub title: String,
    /// App version string.
    #[serde(rename = "Version")]
    pub version: String,
    /// Product type identifier (e.g., "1F" for iPhone app).
    #[serde(rename = "Product Type Identifier")]
    pub product_type_identifier: String,
    /// Number of units.
    #[serde(rename = "Units")]
    pub units: String,
    /// Developer proceeds amount.
    #[serde(rename = "Developer Proceeds")]
    pub developer_proceeds: String,
    /// Currency of proceeds.
    #[serde(rename = "Currency of Proceeds")]
    pub currency_of_proceeds: String,
    /// Begin date of the report period.
    #[serde(rename = "Begin Date")]
    pub begin_date: String,
    /// End date of the report period.
    #[serde(rename = "End Date")]
    pub end_date: String,
    /// Customer's local currency.
    #[serde(rename = "Customer Currency")]
    pub customer_currency: String,
    /// Customer-facing price.
    #[serde(rename = "Customer Price")]
    pub customer_price: String,
    /// Promo code used, if any.
    #[serde(rename = "Promo Code")]
    pub promo_code: String,
    /// Parent app identifier for IAP.
    #[serde(rename = "Parent Identifier")]
    pub parent_identifier: String,
    /// Subscription indicator.
    #[serde(rename = "Subscription")]
    pub subscription: String,
    /// Subscription period.
    #[serde(rename = "Period")]
    pub period: String,
    /// App category.
    #[serde(rename = "Category")]
    pub category: String,
    /// CMB (content/media/bundle) indicator.
    #[serde(rename = "CMB")]
    pub cmb: String,
    /// Device type.
    #[serde(rename = "Device")]
    pub device: String,
    /// Supported platforms.
    #[serde(rename = "Supported Platforms")]
    pub supported_platforms: String,
    /// Proceeds reason.
    #[serde(rename = "Proceeds Reason")]
    pub proceeds_reason: String,
    /// Preserved pricing indicator.
    #[serde(rename = "Preserved Pricing")]
    pub preserved_pricing: String,
    /// Client type.
    #[serde(rename = "Client")]
    pub client: String,
    /// Order type.
    #[serde(rename = "Order Type")]
    pub order_type: String,
}

/// Parses gzip-compressed TSV sales report data into structured rows.
///
/// The App Store Connect API returns sales reports as gzip-compressed
/// tab-separated values. This function decompresses and parses the data.
///
/// # Errors
///
/// Returns an error if gzip decompression fails or TSV parsing fails.
pub fn parse_sales_tsv(data: &[u8]) -> Result<Vec<SalesReportRow>, SalesTsvError> {
    let mut decoder = flate2::read::GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| SalesTsvError::Decompression(e.to_string()))?;

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .flexible(true)
        .from_reader(decompressed.as_slice());

    let mut rows = Vec::new();
    for result in reader.deserialize() {
        let row: SalesReportRow = result.map_err(|e| SalesTsvError::Parse(e.to_string()))?;
        rows.push(row);
    }
    Ok(rows)
}

/// Errors that can occur when parsing sales report TSV data.
#[derive(Debug, thiserror::Error)]
pub enum SalesTsvError {
    /// Failed to decompress gzip data.
    #[error("gzip decompression failed: {0}")]
    Decompression(String),
    /// Failed to parse TSV data.
    #[error("TSV parse error: {0}")]
    Parse(String),
}

#[cfg(test)]
#[path = "sales_tests.rs"]
mod tests;

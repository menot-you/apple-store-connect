//! Tests for sales report TSV parsing.

use super::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Write;

/// Helper: compress a string with gzip.
fn gzip_compress(data: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

/// Minimal TSV header + one row matching SalesReportRow fields.
fn sample_tsv() -> String {
    let header = "Provider\tProvider Country\tSKU\tDeveloper\tTitle\tVersion\t\
                  Product Type Identifier\tUnits\tDeveloper Proceeds\t\
                  Currency of Proceeds\tBegin Date\tEnd Date\tCustomer Currency\t\
                  Customer Price\tPromo Code\tParent Identifier\tSubscription\t\
                  Period\tCategory\tCMB\tDevice\tSupported Platforms\t\
                  Proceeds Reason\tPreserved Pricing\tClient\tOrder Type";
    let row = "APPLE\tUS\tCOOL001\tDev Inc\tMy App\t1.0\t1F\t100\t70.00\t\
               USD\t03/01/2026\t03/31/2026\tUSD\t0.99\t\tcom.example\t\t\t\
               Games\t\tiPhone\tiOS\t\t\tApp Store\tBuy";
    format!("{header}\n{row}\n")
}

#[test]
fn test_parse_sales_tsv_single_row() {
    let compressed = gzip_compress(&sample_tsv());
    let rows = parse_sales_tsv(&compressed).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].provider, "APPLE");
    assert_eq!(rows[0].sku, "COOL001");
    assert_eq!(rows[0].units, "100");
    assert_eq!(rows[0].developer_proceeds, "70.00");
    assert_eq!(rows[0].currency_of_proceeds, "USD");
    assert_eq!(rows[0].title, "My App");
}

#[test]
fn test_parse_sales_tsv_multiple_rows() {
    let header = "Provider\tProvider Country\tSKU\tDeveloper\tTitle\tVersion\t\
                  Product Type Identifier\tUnits\tDeveloper Proceeds\t\
                  Currency of Proceeds\tBegin Date\tEnd Date\tCustomer Currency\t\
                  Customer Price\tPromo Code\tParent Identifier\tSubscription\t\
                  Period\tCategory\tCMB\tDevice\tSupported Platforms\t\
                  Proceeds Reason\tPreserved Pricing\tClient\tOrder Type";
    let row1 = "APPLE\tUS\tSKU1\tDev\tApp1\t1.0\t1F\t50\t35.00\t\
                USD\t03/01/2026\t03/31/2026\tUSD\t0.99\t\t\t\t\tGames\t\t\
                iPhone\tiOS\t\t\tApp Store\tBuy";
    let row2 = "APPLE\tGB\tSKU2\tDev\tApp2\t2.0\t1F\t25\t17.50\t\
                GBP\t03/01/2026\t03/31/2026\tGBP\t0.79\t\t\t\t\tUtil\t\t\
                iPhone\tiOS\t\t\tApp Store\tBuy";
    let tsv = format!("{header}\n{row1}\n{row2}\n");
    let compressed = gzip_compress(&tsv);
    let rows = parse_sales_tsv(&compressed).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].sku, "SKU1");
    assert_eq!(rows[1].sku, "SKU2");
    assert_eq!(rows[1].provider_country, "GB");
}

#[test]
fn test_parse_sales_tsv_invalid_gzip() {
    let result = parse_sales_tsv(b"not-gzip-data");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, SalesTsvError::Decompression(_)));
}

#[test]
fn test_parse_sales_tsv_empty_after_header() {
    let header = "Provider\tProvider Country\tSKU\tDeveloper\tTitle\tVersion\t\
                  Product Type Identifier\tUnits\tDeveloper Proceeds\t\
                  Currency of Proceeds\tBegin Date\tEnd Date\tCustomer Currency\t\
                  Customer Price\tPromo Code\tParent Identifier\tSubscription\t\
                  Period\tCategory\tCMB\tDevice\tSupported Platforms\t\
                  Proceeds Reason\tPreserved Pricing\tClient\tOrder Type";
    let compressed = gzip_compress(&format!("{header}\n"));
    let rows = parse_sales_tsv(&compressed).unwrap();
    assert!(rows.is_empty());
}

#[test]
fn test_sales_report_row_serializes() {
    let compressed = gzip_compress(&sample_tsv());
    let rows = parse_sales_tsv(&compressed).unwrap();
    let json = serde_json::to_string(&rows[0]).unwrap();
    assert!(json.contains("COOL001"));
    assert!(json.contains("70.00"));
}

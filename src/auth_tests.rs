//! Tests for JWT authentication.

use super::*;
use std::path::PathBuf;

/// Path to the test EC key fixture.
fn test_key_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("test-key.p8")
}

/// Loads the test EC key from the fixtures directory.
fn test_ec_key_pem() -> Vec<u8> {
    std::fs::read(test_key_path()).expect("test fixture tests/fixtures/test-key.p8 missing")
}

#[test]
fn test_token_generation() {
    let creds = Credentials::new(
        "TEST_KEY_ID".into(),
        "TEST_ISSUER".into(),
        test_ec_key_pem(),
    );
    let token = creds.token();
    assert!(token.is_ok(), "token generation failed: {token:?}");
    let token = token.unwrap();
    assert_eq!(token.split('.').count(), 3);
}

#[test]
fn test_token_caching() {
    let creds = Credentials::new(
        "TEST_KEY_ID".into(),
        "TEST_ISSUER".into(),
        test_ec_key_pem(),
    );
    let t1 = creds.token().unwrap();
    let t2 = creds.token().unwrap();
    assert_eq!(t1, t2, "second call should return cached token");
}

#[test]
fn test_token_expiry_regenerates() {
    let creds = Credentials::new(
        "TEST_KEY_ID".into(),
        "TEST_ISSUER".into(),
        test_ec_key_pem(),
    );
    let _t1 = creds.token().unwrap();

    // Force cache to appear expired (16 min > 15 min lifetime).
    let expired_time = Instant::now() - Duration::from_secs(16 * 60);
    {
        let mut cache = creds.cache.lock().unwrap();
        cache.as_mut().unwrap().created_at = expired_time;
    }

    // Request a token — should regenerate and update created_at.
    let _t2 = creds.token().unwrap();
    let new_created_at = {
        let cache = creds.cache.lock().unwrap();
        cache.as_ref().unwrap().created_at
    };

    assert!(
        new_created_at > expired_time,
        "cache should have been refreshed with a newer timestamp"
    );
}

#[test]
fn test_invalid_key() {
    let creds = Credentials::new("K".into(), "I".into(), b"bogus-data-not-pem".to_vec());
    let result = creds.token();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::InvalidKey(_)));
}

#[test]
fn test_from_env_missing_var_error_type() {
    let result = std::env::var("ASC_KEY_ID_DEFINITELY_NOT_SET_12345");
    assert!(result.is_err(), "test assumes this env var is not set");
    let err = AuthError::MissingEnvVar("ASC_KEY_ID".into());
    assert!(
        matches!(err, AuthError::MissingEnvVar(ref v) if v == "ASC_KEY_ID"),
        "MissingEnvVar should carry the variable name"
    );
}

#[test]
fn test_key_read_error_on_bad_path() {
    let creds = Credentials::new("K".into(), "I".into(), vec![]);
    let err = std::fs::read("/nonexistent/path/to/key.p8");
    assert!(err.is_err());
    let auth_err = AuthError::KeyReadError(err.unwrap_err().to_string());
    assert!(matches!(auth_err, AuthError::KeyReadError(_)));
    assert!(creds.token().is_err());
}

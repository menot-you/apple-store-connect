//! JWT authentication for the App Store Connect API.
//!
//! Generates ES256-signed tokens per Apple's specification:
//! - Header: `{ "alg": "ES256", "kid": "<KEY_ID>", "typ": "JWT" }`
//! - Payload: `{ "iss": "<ISSUER_ID>", "iat": <now>, "exp": <now+20m>, "aud": "appstoreconnect-v1" }`

use std::sync::Mutex;
use std::time::{Duration, Instant};

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::Serialize;

/// Errors that can occur during authentication.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// A required environment variable is missing.
    #[error("missing environment variable: {0}")]
    MissingEnvVar(String),
    /// Failed to read the key file from disk.
    #[error("failed to read key file: {0}")]
    KeyReadError(String),
    /// The key data could not be parsed as a valid PEM EC key.
    #[error("invalid key data: {0}")]
    InvalidKey(String),
    /// JWT encoding failed.
    #[error("jwt encoding failed: {0}")]
    EncodingError(String),
}

/// JWT claims for App Store Connect API.
#[derive(Debug, Serialize)]
struct Claims {
    iss: String,
    iat: i64,
    exp: i64,
    aud: String,
}

/// Cached token with its creation instant.
struct CachedToken {
    token: String,
    created_at: Instant,
}

/// App Store Connect API credentials for JWT generation.
pub struct Credentials {
    /// The Key ID from App Store Connect.
    key_id: String,
    /// The Issuer ID from App Store Connect.
    issuer_id: String,
    /// The PEM-encoded ES256 key bytes.
    key_pem: Vec<u8>,
    /// Cached token with expiry tracking.
    cache: Mutex<Option<CachedToken>>,
}

/// Duration a cached token is considered valid (15 minutes).
/// Apple allows 20 minutes max; we use 15 for a 5-minute buffer.
const TOKEN_LIFETIME: Duration = Duration::from_secs(15 * 60);

impl Credentials {
    /// Creates credentials from explicit values.
    pub fn new(key_id: String, issuer_id: String, key_pem: Vec<u8>) -> Self {
        Self {
            key_id,
            issuer_id,
            key_pem,
            cache: Mutex::new(None),
        }
    }

    /// Creates credentials from environment variables.
    ///
    /// Reads `ASC_KEY_ID`, `ASC_ISSUER_ID`, and `ASC_PRIVATE_KEY_PATH`.
    pub fn from_env() -> Result<Self, AuthError> {
        let key_id = std::env::var("ASC_KEY_ID")
            .map_err(|_| AuthError::MissingEnvVar("ASC_KEY_ID".into()))?;
        let issuer_id = std::env::var("ASC_ISSUER_ID")
            .map_err(|_| AuthError::MissingEnvVar("ASC_ISSUER_ID".into()))?;
        let key_path = std::env::var("ASC_PRIVATE_KEY_PATH")
            .map_err(|_| AuthError::MissingEnvVar("ASC_PRIVATE_KEY_PATH".into()))?;
        let key_pem =
            std::fs::read(&key_path).map_err(|e| AuthError::KeyReadError(e.to_string()))?;
        Ok(Self::new(key_id, issuer_id, key_pem))
    }

    /// Generates a JWT token, reusing a cached one if still valid.
    pub fn token(&self) -> Result<String, AuthError> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| AuthError::EncodingError("token cache lock poisoned".into()))?;
        if let Some(cached) = cache.as_ref() {
            if cached.created_at.elapsed() < TOKEN_LIFETIME {
                return Ok(cached.token.clone());
            }
        }
        let token = self.generate_token()?;
        *cache = Some(CachedToken {
            token: token.clone(),
            created_at: Instant::now(),
        });
        Ok(token)
    }

    /// Generates a fresh JWT token (not cached).
    fn generate_token(&self) -> Result<String, AuthError> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            iss: self.issuer_id.clone(),
            iat: now,
            exp: now + 20 * 60,
            aud: "appstoreconnect-v1".into(),
        };
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.key_id.clone());
        header.typ = Some("JWT".into());
        let key = EncodingKey::from_ec_pem(&self.key_pem)
            .map_err(|e| AuthError::InvalidKey(e.to_string()))?;
        encode(&header, &claims, &key).map_err(|e| AuthError::EncodingError(e.to_string()))
    }
}

#[cfg(test)]
#[path = "auth_tests.rs"]
mod tests;

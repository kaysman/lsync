use crate::utils::constants::*;
use crate::utils::env::*;
use crate::utils::logging::*;
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::Error as HyperError;
use hyper_rustls::HttpsConnector;
use rustls::crypto::ring::default_provider;
use rustls::crypto::CryptoProvider;
use sheets4::{yup_oauth2, Sheets};

use sheets4::hyper_util::client::legacy::{connect::HttpConnector, Client as HyperLegacyClient};

type AppConnector = HttpsConnector<HttpConnector>;

type SheetsClientType = HyperLegacyClient<AppConnector, BoxBody<Bytes, HyperError>>;

pub struct GoogleSheetApiClient {}

impl GoogleSheetApiClient {
    pub async fn get_client() -> Sheets<AppConnector> {
        let provider: CryptoProvider = default_provider();
        provider.install_default().unwrap();

        let client_id = get_client_id();
        let client_secret = get_client_secret();

        let client_secret = yup_oauth2::ApplicationSecret {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            auth_uri: AUTH_URI.to_string(),
            token_uri: TOKEN_URI.to_string(),
            redirect_uris: vec![REDIRECT_URI_1.to_string()],
            ..Default::default()
        };

        log_info("Starting Google OAuth flow...");

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            client_secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk(".localyze/credentials.json")
        .build()
        .await
        .unwrap_or_else(|e| {
            log_error(&format!("OAuth failed: {}", e));
            std::process::exit(1);
        });

        let http_connector: AppConnector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .build();

        let client: SheetsClientType =
            HyperLegacyClient::builder(sheets4::hyper_util::rt::TokioExecutor::new())
                .build(http_connector);

        Sheets::new(client, auth)
    }
}

//! Unofficial Rust library for the [Wyre](https://www.sendwyre.com/) payment
//! API.
//!
//! Documentation: <https://docs.sendwyre.com/>

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

use futures03::compat::Future01CompatExt;
use reqwest::r#async::{Body as ReqwestBody, Client as ReqwestClient};
use reqwest::StatusCode;
use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;

mod account;
mod common;
mod environment;
mod error;
mod payment_method;
mod transfer;

pub use account::*;
pub use common::*;
pub use environment::*;
pub use error::*;
pub use payment_method::*;
pub use transfer::*;

/// A client that can be used to access the Wyre API
#[derive(Debug, Clone)]
pub struct Client {
    http_client: ReqwestClient,
    environment: Environment,
    _api_key: SecretString,
    api_secret: SecretString,
}

impl Client {
    /// Creates a new client
    #[must_use]
    pub fn new(
        api_key: SecretString,
        api_secret: SecretString,
        environment: Environment,
    ) -> Client {
        Client {
            http_client: ReqwestClient::new(),
            environment,
            _api_key: api_key,
            api_secret,
        }
    }

    /// Creates a new client from environment variables:
    /// - `WYRE_API_KEY`
    /// - `WYRE_API_SECRET`
    /// - `WYRE_ENVIRONMENT`
    pub fn from_env() -> Result<Client, ClientFromEnvironmentError> {
        use ClientFromEnvironmentError::*;

        let api_key = std::env::var("WYRE_API_KEY").map_err(|_| MissingApiKey)?;
        let api_secret = std::env::var("WYRE_API_SECRET").map_err(|_| MissingApiSecret)?;
        let environment = std::env::var("WYRE_ENVIRONMENT").map_err(|_| MissingEnvironment)?;

        Ok(Client::new(
            SecretString::new(api_key),
            SecretString::new(api_secret),
            environment.parse()?,
        ))
    }

    /// See [Get Master Account](https://docs.sendwyre.com/docs/get-master-account).
    pub async fn get_master_account(&self) -> Result<Transfer, Error> {
        let url = format!("{}/v2/account", self.environment.api_url());

        let mut response = self
            .http_client
            .get(&url)
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .compat()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().compat().await?),
            _ => Err(Error::Api(response.json().compat().await?)),
        }
    }

    /// See [Create Account](https://docs.sendwyre.com/docs/create-account)
    pub async fn create_account(&self, body: CreateAccount) -> Result<Account, Error> {
        let url = format!("{}/v3/accounts", self.environment.api_url());

        let mut response = self
            .http_client
            .post(&url)
            .bearer_auth(self.api_secret.expose_secret())
            .json(&body)
            .send()
            .compat()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().compat().await?),
            _ => Err(Error::Api(response.json().compat().await?)),
        }
    }

    /// See [Upload Document](https://docs.sendwyre.com/docs/upload-document)
    pub async fn upload_document<D: Into<ReqwestBody>>(
        &self,
        account_id: String,
        document: UploadDocument<D>,
    ) -> Result<Account, Error> {
        let url = format!(
            "{}/v3/accounts/{}/{}",
            self.environment.api_url(),
            account_id,
            document.field_id
        );

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct UploadDocumentQueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            document_type: Option<DocumentType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            document_sub_type: Option<DocumentSubType>,
            masquerade_as: String,
        }

        let mut response = self
            .http_client
            .post(&url)
            .query(&UploadDocumentQueryParams {
                document_type: document.document_type,
                document_sub_type: document.document_sub_type,
                masquerade_as: account_id,
            })
            .bearer_auth(self.api_secret.expose_secret())
            .header(reqwest::header::CONTENT_TYPE, document.content_type)
            .body(document.document)
            .send()
            .compat()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().compat().await?),
            _ => Err(Error::Api(response.json().compat().await?)),
        }
    }

    /// See [ACH - Create Payment Method](https://docs.sendwyre.com/docs/ach-create-payment-method-processor-token-model).
    pub async fn create_ach_payment_method(
        &self,
        body: CreateAchPaymentMethod,
        masquerade: Option<String>,
    ) -> Result<PaymentMethod, Error> {
        let url = format!("{}/v2/paymentMethods", self.environment.api_url());

        let mut response = self
            .http_client
            .post(&url)
            .query(&[("masqueradeAs", masquerade.unwrap_or_default())])
            .bearer_auth(self.api_secret.expose_secret())
            .json(&body)
            .send()
            .compat()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().compat().await?),
            _ => Err(Error::Api(response.json().compat().await?)),
        }
    }

    /// See [List Payment Methods](https://docs.sendwyre.com/docs/list-payment-methods).
    pub async fn get_payment_methods(
        &self,
        masquerade: Option<String>,
        offset: usize,
        limit: usize,
    ) -> Result<PaymentMethodList, Error> {
        let url = format!("{}/v2/paymentMethods", self.environment.api_url());

        let mut response = self
            .http_client
            .get(&url)
            .query(&[
                ("offset", offset.to_string()),
                ("limit", limit.to_string()),
                ("masqueradeAs", masquerade.unwrap_or_default()),
            ])
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .compat()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().compat().await?),
            _ => Err(Error::Api(response.json().compat().await?)),
        }
    }

    /// See [Create Transfer](https://docs.sendwyre.com/docs/create-transfer).
    pub async fn create_transfer(
        &self,
        body: CreateTransfer,
        masquerade: Option<String>,
    ) -> Result<Transfer, Error> {
        let url = format!("{}/v3/transfers", self.environment.api_url());

        let mut response = self
            .http_client
            .post(&url)
            .query(&[("masqueradeAs", masquerade.unwrap_or_default())])
            .bearer_auth(self.api_secret.expose_secret())
            .json(&body)
            .send()
            .compat()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().compat().await?),
            _ => Err(Error::Api(response.json().compat().await?)),
        }
    }

    /// See [Get Transfer](https://docs.sendwyre.com/docs/get-transfer).
    pub async fn get_transfer(
        &self,
        transfer_id: String,
        masquerade: Option<String>,
    ) -> Result<Transfer, Error> {
        let url = format!(
            "{}/v3/transfers/{}",
            self.environment.api_url(),
            transfer_id
        );

        let mut response = self
            .http_client
            .get(&url)
            .query(&[("masqueradeAs", masquerade.unwrap_or_default())])
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .compat()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().compat().await?),
            _ => Err(Error::Api(response.json().compat().await?)),
        }
    }
}

/// Error received from [Client::from_env]
pub enum ClientFromEnvironmentError {
    /// No `WYRE_API_KEY` variable
    MissingApiKey,

    /// No `WYRE_API_SECRET` variable
    MissingApiSecret,

    /// No `WYRE_ENVIRONMENT` variable
    MissingEnvironment,

    /// The `WYRE_ENVIRONMENT` variable didn't match an expected value.
    EnvironmentParseError(EnvironmentParseError),
}

impl From<EnvironmentParseError> for ClientFromEnvironmentError {
    fn from(error: EnvironmentParseError) -> Self {
        ClientFromEnvironmentError::EnvironmentParseError(error)
    }
}

#[cfg(test)]
mod tests {
    use futures03::{FutureExt, TryFutureExt};
    use secrecy::SecretString;
    use tokio01::runtime::Runtime as Runtime01;
    use tokio10::runtime::Runtime as Runtime10;

    use crate as wyre;

    #[test]
    fn can_create_ach_transfer_via_plaid() {
        dotenv::dotenv().unwrap();

        let rt_10 = Runtime10::new().unwrap();
        let plaid_client = plaid::Client::new(
            std::env::var("PLAID_CLIENT_ID").unwrap(),
            std::env::var("PLAID_SECRET").unwrap(),
            plaid::client::Environment::Sandbox,
        );

        let public_token_response = rt_10
            .block_on(plaid_client.create_sandbox_public_token("ins_1", &["auth", "identity"]))
            .unwrap();

        let exchange_token_response = rt_10
            .block_on(plaid_client.exchange_public_token(&public_token_response.public_token))
            .unwrap();

        let accounts_response = rt_10
            .block_on(plaid_client.get_accounts(&exchange_token_response.access_token, None))
            .unwrap();

        let processor_token_response = rt_10
            .block_on(plaid_client.create_processor_token(
                &exchange_token_response.access_token,
                &accounts_response.accounts[0].account_id,
                "wyre",
            ))
            .unwrap();

        let mut rt_01 = Runtime01::new().unwrap();
        let wyre_client = wyre::Client::new(
            SecretString::new(std::env::var("WYRE_API_KEY").unwrap()),
            SecretString::new(std::env::var("WYRE_API_SECRET").unwrap()),
            wyre::Environment::Test,
        );
        let wyre_client: &'static _ = Box::leak(Box::new(wyre_client));

        let account = rt_01
            .block_on(
                wyre_client
                    .create_account(wyre::CreateAccount {
                        kind: wyre::AccountType::Individual,
                        country: "US".into(),
                        profile_fields: vec![
                            wyre::CreateProfileField {
                                field_id: wyre::ProfileFieldId::IndividualLegalName,
                                value: wyre::ProfileFieldType::String(Some("Alice Loyd".into())),
                            },
                            wyre::CreateProfileField {
                                field_id: wyre::ProfileFieldId::IndividualResidenceAddress,
                                value: wyre::ProfileFieldType::Address(Some(wyre::Address {
                                    street1: Some("7819 E. Stonybrook St.".into()),
                                    street2: None,
                                    city: Some("Seattle".into()),
                                    state: Some("WA".into()),
                                    postal_code: Some("98111".into()),
                                    country: Some("US".into()),
                                })),
                            },
                            wyre::CreateProfileField {
                                field_id: wyre::ProfileFieldId::IndividualCellphoneNumber,
                                value: wyre::ProfileFieldType::Cellphone(Some(
                                    "+12062108021".into(),
                                )),
                            },
                            wyre::CreateProfileField {
                                field_id: wyre::ProfileFieldId::IndividualEmail,
                                value: wyre::ProfileFieldType::Email(Some(
                                    "test@example.com".into(),
                                )),
                            },
                            wyre::CreateProfileField {
                                field_id: wyre::ProfileFieldId::IndividualDateOfBirth,
                                value: wyre::ProfileFieldType::Date(Some("1990-09-24".into())),
                            },
                            wyre::CreateProfileField {
                                field_id: wyre::ProfileFieldId::IndividualSsn,
                                value: wyre::ProfileFieldType::Date(Some("123-45-6789".into())),
                            },
                        ],
                        referrer_account_id: None,
                        subaccount: Some(true),
                        disable_email: Some(true),
                    })
                    .boxed()
                    .compat(),
            )
            .unwrap();

        // the Wyre test environment doesn't validate KYC but it does check
        // that all the required fields are set and for document uploads the
        // file must be valid.
        //
        // from: https://stackoverflow.com/questions/2253404/what-is-the-smallest-valid-jpeg-file-size-in-bytes
        let smallest_jpeg: &[u8] = b"\
            \xff\xd8\xff\xe0\x00\x10\x4a\x46\x49\x46\x00\x01\x01\x01\x00\x48\x00\
            \x48\x00\x00\xff\xdb\x00\x43\x00\x03\x02\x02\x02\x02\x02\x03\x02\x02\
            \x02\x03\x03\x03\x03\x04\x06\x04\x04\x04\x04\x04\x08\x06\x06\x05\x06\
            \x09\x08\x0a\x0a\x09\x08\x09\x09\x0a\x0c\x0f\x0c\x0a\x0b\x0e\x0b\x09\
            \x09\x0d\x11\x0d\x0e\x0f\x10\x10\x11\x10\x0a\x0c\x12\x13\x12\x10\x13\
            \x0f\x10\x10\x10\xff\xc9\x00\x0b\x08\x00\x01\x00\x01\x01\x01\x11\x00\
            \xff\xcc\x00\x06\x00\x10\x10\x05\xff\xda\x00\x08\x01\x01\x00\x00\x3f\
            \x00\xd2\xcf\x20\xff\xd9";

        let _upload_front = rt_01
            .block_on(
                wyre_client
                    .upload_document(
                        account.id.clone(),
                        wyre::UploadDocument {
                            field_id: wyre::ProfileFieldId::IndividualGovernmentId,
                            document_type: Some(wyre::DocumentType::GovtId),
                            document_sub_type: Some(wyre::DocumentSubType::Front),
                            document: smallest_jpeg,
                            content_type: "image/jpeg".to_string(),
                        },
                    )
                    .boxed()
                    .compat(),
            )
            .unwrap();

        let _upload_back = rt_01
            .block_on(
                wyre_client
                    .upload_document(
                        account.id.clone(),
                        wyre::UploadDocument {
                            field_id: wyre::ProfileFieldId::IndividualGovernmentId,
                            document_type: Some(wyre::DocumentType::GovtId),
                            document_sub_type: Some(wyre::DocumentSubType::Back),
                            document: smallest_jpeg,
                            content_type: "image/jpeg".to_string(),
                        },
                    )
                    .boxed()
                    .compat(),
            )
            .unwrap();

        let _ach_payment_method = rt_01
            .block_on(
                wyre_client
                    .create_ach_payment_method(
                        wyre::CreateAchPaymentMethod {
                            plaid_processor_token: processor_token_response.processor_token,
                            payment_method_type: wyre::PaymentMethodType::LocalTransfer,
                            country: wyre::AchPaymentMethodCountry::US,
                        },
                        Some(account.id.clone()),
                    )
                    .boxed()
                    .compat(),
            )
            .unwrap();

        // give the Wyre test environment a chance to activate the payment
        // method
        std::thread::sleep(std::time::Duration::from_secs(30));

        let payment_methods = rt_01
            .block_on(
                wyre_client
                    .get_payment_methods(Some(account.id.clone()), 0, 10)
                    .boxed()
                    .compat(),
            )
            .unwrap();

        assert_eq!(payment_methods.records_total, 1);
        assert_eq!(
            payment_methods.data[0].status,
            wyre::PaymentMethodStatus::Active
        );

        let created_transfer = rt_01
            .block_on(
                wyre_client
                    .create_transfer(
                        wyre::CreateTransfer {
                            source: format!("paymentmethod:{}:ach", payment_methods.data[0].id),
                            source_currency: wyre::Currency::USD,
                            source_amount: Some(20.00),
                            dest: "ethereum:0xc12fae05cbe72a501540f260d6c49ddc6f9d9f4d".to_string(),
                            dest_currency: Some(wyre::Currency::USDC),
                            dest_amount: None,
                            message: Some("test transfer".into()),
                            notify_url: None,
                            auto_confirm: Some(true),
                            custom_id: None,
                            amount_includes_fees: Some(false),
                            preview: Some(false),
                            mute_messages: Some(true),
                        },
                        Some(account.id.clone()),
                    )
                    .boxed()
                    .compat(),
            )
            .unwrap();

        let _transfer = rt_01
            .block_on(
                wyre_client
                    .get_transfer(created_transfer.id.clone(), Some(account.id.clone()))
                    .boxed()
                    .compat(),
            )
            .unwrap();
    }
}

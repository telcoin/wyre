//! Unofficial Rust library for the [Wyre](https://www.sendwyre.com/) payment
//! API.
//!
//! Documentation: <https://docs.sendwyre.com/>

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

use reqwest::StatusCode;
use reqwest::{Body as ReqwestBody, Client as ReqwestClient};
use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;

mod account;
mod common;
mod environment;
mod error;
mod payment_method;
mod transfer;
mod user;

pub use account::*;
pub use common::*;
pub use environment::*;
pub use error::*;
pub use payment_method::*;
pub use transfer::*;
pub use user::*;

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
    pub async fn get_master_account(&self) -> Result<MasterAccount, Error> {
        let url = format!("{}/v2/account", self.environment.api_url());

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Create Account](https://docs.sendwyre.com/docs/create-account)
    pub async fn create_account(&self, body: CreateAccount) -> Result<Account, Error> {
        let url = format!("{}/v3/accounts", self.environment.api_url());

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(self.api_secret.expose_secret())
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Get Account](https://docs.sendwyre.com/docs/get-account).
    pub async fn get_account(
        &self,
        account_id: String,
        masquerade: Option<SystemResourceName>,
    ) -> Result<Account, Error> {
        let url = format!("{}/v3/accounts/{}", self.environment.api_url(), account_id);

        let response = self
            .http_client
            .get(&url)
            .query(&[(
                "masqueradeAs",
                masquerade
                    .map(|srn| srn.to_string())
                    .unwrap_or_else(String::new),
            )])
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Update Account](https://docs.sendwyre.com/docs/submit-account-info).
    pub async fn update_account(
        &self,
        account_id: String,
        update: UpdateAccount,
        masquerade: Option<SystemResourceName>,
    ) -> Result<Account, Error> {
        let url = format!("{}/v3/accounts/{}", self.environment.api_url(), account_id);

        let response = self
            .http_client
            .post(&url)
            .query(&[(
                "masqueradeAs",
                masquerade
                    .map(|srn| srn.to_string())
                    .unwrap_or_else(String::new),
            )])
            .bearer_auth(self.api_secret.expose_secret())
            .json(&update)
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Upload Document](https://docs.sendwyre.com/docs/upload-document)
    pub async fn upload_document<D: Into<ReqwestBody>>(
        &self,
        account_id: String,
        document: UploadDocument<D>,
        masquerade: Option<SystemResourceName>,
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
            masquerade_as: Option<String>,
        }

        let response = self
            .http_client
            .post(&url)
            .query(&UploadDocumentQueryParams {
                document_type: document.document_type,
                document_sub_type: document.document_sub_type,
                masquerade_as: masquerade.as_ref().map(ToString::to_string),
            })
            .bearer_auth(self.api_secret.expose_secret())
            .header(reqwest::header::CONTENT_TYPE, document.content_type)
            .body(document.document)
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [ACH - Create Payment Method](https://docs.sendwyre.com/docs/ach-create-payment-method-processor-token-model).
    pub async fn create_ach_payment_method(
        &self,
        body: CreateAchPaymentMethod,
        masquerade: Option<SystemResourceName>,
    ) -> Result<PaymentMethod, Error> {
        let url = format!("{}/v2/paymentMethods", self.environment.api_url());

        let response = self
            .http_client
            .post(&url)
            .query(&[(
                "masqueradeAs",
                masquerade
                    .map(|srn| srn.to_string())
                    .unwrap_or_else(String::new),
            )])
            .bearer_auth(self.api_secret.expose_secret())
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [List Payment Methods](https://docs.sendwyre.com/docs/list-payment-methods).
    pub async fn get_payment_methods(
        &self,
        masquerade: Option<SystemResourceName>,
        offset: usize,
        limit: usize,
    ) -> Result<PaymentMethodList, Error> {
        let url = format!("{}/v2/paymentMethods", self.environment.api_url());

        let response = self
            .http_client
            .get(&url)
            .query(&[
                ("offset", offset.to_string()),
                ("limit", limit.to_string()),
                (
                    "masqueradeAs",
                    masquerade
                        .map(|srn| srn.to_string())
                        .unwrap_or_else(String::new),
                ),
            ])
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Create Transfer](https://docs.sendwyre.com/docs/create-transfer).
    pub async fn create_transfer(
        &self,
        body: CreateTransfer,
        masquerade: Option<SystemResourceName>,
    ) -> Result<Transfer, Error> {
        let url = format!("{}/v3/transfers", self.environment.api_url());

        let response = self
            .http_client
            .post(&url)
            .query(&[(
                "masqueradeAs",
                masquerade
                    .map(|srn| srn.to_string())
                    .unwrap_or_else(String::new),
            )])
            .bearer_auth(self.api_secret.expose_secret())
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Get Transfer](https://docs.sendwyre.com/docs/get-transfer).
    pub async fn get_transfer(
        &self,
        transfer_id: String,
        masquerade: Option<SystemResourceName>,
    ) -> Result<Transfer, Error> {
        let url = format!(
            "{}/v3/transfers/{}",
            self.environment.api_url(),
            transfer_id
        );

        let response = self
            .http_client
            .get(&url)
            .query(&[(
                "masqueradeAs",
                masquerade
                    .map(|srn| srn.to_string())
                    .unwrap_or_else(String::new),
            )])
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Create User](https://docs.sendwyre.com/reference/create-user).
    pub async fn create_user(&self, req: ModifyUser) -> Result<User, Error> {
        let url = format!("{}/v3/users", self.environment.api_url());

        let response = self
            .http_client
            .post(&url)
            .json(&req)
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Get User](https://docs.sendwyre.com/reference/get-user)
    ///
    /// /// Masquerade should be the User SystemResourceName
    pub async fn get_user(
        &self,
        user_id: String,
        scope: UserScope,
        masquerade: Option<SystemResourceName>,
    ) -> Result<User, Error> {
        let url = format!("{}/v3/users/{}", self.environment.api_url(), user_id);

        let response = self
            .http_client
            .get(&url)
            .query(&[
                (
                    "masqueradeAs",
                    masquerade
                        .map(|srn| srn.to_string())
                        .unwrap_or_else(String::new),
                ),
                ("scopes", scope.to_string()),
            ])
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Update User](https://docs.sendwyre.com/reference/upload-user-data)
    ///
    /// Masquerade should be the User SystemResourceName
    pub async fn update_user(
        &self,
        user_id: String,
        req: ModifyUser,
        masquerade: Option<SystemResourceName>,
    ) -> Result<User, Error> {
        let url = format!("{}/v3/users/{}", self.environment.api_url(), user_id);

        let response = self
            .http_client
            .post(&url)
            .query(&[(
                "masqueradeAs",
                masquerade
                    .map(|srn| srn.to_string())
                    .unwrap_or_else(String::new),
            )])
            .json(&req)
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Get KYC Onboarding URL](https://docs.sendwyre.com/reference/get-onboarding-url)
    ///
    /// Retrieve URL for users to go through higher limit KYC onboarding
    pub async fn get_kyc_onboarding_url(
        &self,
        user_id: String,
        country_code: Option<String>,
    ) -> Result<OnboardingUrl, Error> {
        let url = format!(
            "{}/v3/users/{}/onboarding",
            self.environment.api_url(),
            user_id
        );

        let response = self
            .http_client
            .get(&url)
            .query(&["countryCode", country_code.as_deref().unwrap_or_default()])
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// See [Update User](https://docs.sendwyre.com/reference/upload-user-data)
    ///
    /// Masquerade should be the User SystemResourceName
    pub async fn update_user_document<D: Into<ReqwestBody>>(
        &self,
        user_id: String,
        req: UserDocumentUpload<D>,
    ) -> Result<User, Error> {
        let url = format!(
            "{}/v3/users/{}/{}",
            self.environment.api_url(),
            user_id,
            req.field_id
        );

        let response = self
            .http_client
            .post(&url)
            .query(&[("docType", req.doc_type)])
            .header(reqwest::header::CONTENT_TYPE, req.content_type)
            .body(req.document)
            .bearer_auth(self.api_secret.expose_secret())
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
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
    use std::{collections::HashMap, convert::TryFrom};

    use bigdecimal::BigDecimal;
    use futures03::{FutureExt, TryFutureExt};
    use tokio01::runtime::Runtime as Runtime01;
    use tokio10::runtime::Runtime as Runtime10;

    use crate::{
        self as wyre, Address, ModifyUser, SystemResourceName as SRN, UserFieldId, UserFieldStatus,
        UserFieldType, UserScope, UserStatus,
    };

    fn client_from_env() -> wyre::Client {
        use std::env;
        dotenv::dotenv().unwrap();

        wyre::Client::new(
            env::var("WYRE_API_KEY").unwrap().into(),
            env::var("WYRE_API_SECRET").unwrap().into(),
            wyre::Environment::Test,
        )
    }

    fn all_fields() -> HashMap<UserFieldId, UserFieldType> {
        let fields = vec![
            (
                UserFieldId::FirstName,
                UserFieldType::String(Some("John".to_owned())),
            ),
            (
                UserFieldId::LastName,
                UserFieldType::String(Some("Smith".to_owned())),
            ),
            (
                UserFieldId::ResidenceAddress,
                UserFieldType::Address(Some(Address {
                    street1: Some("1234 Sesame Street".to_owned()),
                    street2: Some("Apt 34".to_owned()),
                    city: Some("Hollywood".to_owned()),
                    state: Some("CA".to_owned()),
                    postal_code: Some("90210".to_owned()),
                    country: Some("US".to_owned()),
                })),
            ),
            (
                UserFieldId::DateOfBirth,
                UserFieldType::String(Some("1990-03-02".to_owned())),
            ),
            (
                UserFieldId::Email,
                UserFieldType::String(Some("email@website.com".to_owned())),
            ),
            // It doesn't like the cell phone for some reason
            // (
            //     UserFieldId::Cellphone,
            //     UserFieldType::String(Some("+15554445555".to_owned())),
            // ),
        ];
        fields.into_iter().collect()
    }

    #[test]
    fn create_user_all_fields() {
        let mod_user = ModifyUser {
            blockchains: vec![],
            immediate: false,
            fields: all_fields(),
            scopes: vec![UserScope::Transfer],
        };

        let client = client_from_env();
        let runtime = Runtime10::new().unwrap();

        let res = runtime.block_on(client.create_user(mod_user)).unwrap();

        assert_eq!(
            res.status,
            UserStatus::Approved,
            "User was not approved with all fields present"
        );
        res.fields
            .iter()
            .for_each(|(_, field)| assert_eq!(field.status, UserFieldStatus::Submitted));
    }

    #[test]
    fn create_user_immediate() {
        let mod_user = ModifyUser {
            blockchains: vec![],
            immediate: true,
            fields: all_fields(),
            scopes: vec![UserScope::Transfer],
        };

        let client = client_from_env();
        let runtime = Runtime10::new().unwrap();

        let res = runtime.block_on(client.create_user(mod_user)).unwrap();

        assert_eq!(res.status, UserStatus::Pending);
    }

    #[test]
    fn update_user() {
        let client = client_from_env();
        let runtime = Runtime10::new().unwrap();

        // initial empty user
        let mut mod_user = ModifyUser {
            blockchains: Default::default(),
            immediate: false,
            fields: Default::default(),
            scopes: vec![UserScope::Transfer],
        };

        let res = runtime
            .block_on(client.create_user(mod_user.clone()))
            .unwrap();

        assert_ne!(res.status, UserStatus::Approved);
        res.fields
            .iter()
            .for_each(|(_, field)| assert_eq!(field.status, UserFieldStatus::Open));

        // update with all but last name
        let mut fields = all_fields();
        let last_name = fields.remove_entry(&UserFieldId::LastName).unwrap();
        mod_user.fields = fields;

        let res = runtime
            .block_on(client.update_user(res.id.clone(), mod_user.clone(), Some(SRN::User(res.id))))
            .unwrap();

        assert_ne!(res.status, UserStatus::Approved);
        res.fields.iter().for_each(|(id, field)| match id {
            UserFieldId::LastName => assert_eq!(field.status, UserFieldStatus::Open),
            _ => assert_eq!(field.status, UserFieldStatus::Submitted),
        });

        // update last name
        let mut last_name_map = HashMap::<UserFieldId, UserFieldType>::default();
        last_name_map.insert(last_name.0, last_name.1);
        mod_user.fields = last_name_map;

        let res = runtime
            .block_on(client.update_user(res.id.clone(), mod_user, Some(SRN::User(res.id))))
            .unwrap();

        assert_eq!(res.status, UserStatus::Approved);
        res.fields
            .iter()
            .for_each(|(_, field)| assert_eq!(field.status, UserFieldStatus::Submitted));
    }

    #[test]
    fn get_user() {
        let client = client_from_env();
        let runtime = Runtime10::new().unwrap();

        // initial user
        let mut fields = all_fields();
        _ = fields.remove_entry(&UserFieldId::LastName).unwrap();

        let scope = UserScope::Transfer;

        let mod_user = ModifyUser {
            blockchains: Default::default(),
            immediate: false,
            fields: Default::default(),
            scopes: vec![scope.clone()],
        };

        let mut initial_user = runtime.block_on(client.create_user(mod_user)).unwrap();

        let gotten_user = runtime
            .block_on(client.get_user(
                initial_user.id.clone(),
                scope,
                Some(SRN::User(initial_user.id.clone())),
            ))
            .unwrap();

        // The status may occasionally be "Pending" depending on how quickly the initial User was returned,
        // but we're going to ignore that since it's a small technicality
        initial_user.status = gotten_user.status;

        assert_eq!(initial_user, gotten_user);
    }

    #[test]
    fn user_serde() {
        use super::*;
        serde_json::from_str::<User>(
            r#"
            {
                "id": "US_48MBN7LX9VY",
                "status": "OPEN",
                "partnerId": "PT_BWLU6B2W3BX",
                "type": "INDIVIDUAL",
                "createdAt": 1654635321327,
                "depositAddresses": {},
                "totalBalances": {},
                "availableBalances": {},
                "fields": {
                  "firstName": {
                    "value": "John",
                    "error": null,
                    "status": "SUBMITTED"
                  },
                  "lastName": {
                    "value": "Smith",
                    "error": null,
                    "status": "SUBMITTED"
                  },
                  "dateOfBirth": {
                    "value": null,
                    "error": null,
                    "status": "OPEN"
                  },
                  "residenceAddress": {
                    "value": {
                      "street1": "123 Sesame St",
                      "city": "New York City",
                      "state": "New York",
                      "postalCode": "10128",
                      "country": "US"
                    },
                    "error": null,
                    "status": "SUBMITTED"
                  }
                }
              }
            "#,
        )
        .unwrap();
    }

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
        let wyre_client = client_from_env();
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
                        Some(SRN::Account(account.id.clone())),
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
                        Some(SRN::Account(account.id.clone())),
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
                        Some(SRN::Account(account.id.clone())),
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
                    .get_payment_methods(Some(SRN::Account(account.id.clone())), 0, 10)
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
                            source: SRN::AchPaymentMethod(payment_methods.data[0].id.clone()),
                            source_currency: wyre::Currency::USD,
                            source_amount: Some(BigDecimal::try_from(20.00).unwrap()),
                            dest: SRN::Ethereum(
                                "0xc12fae05cbe72a501540f260d6c49ddc6f9d9f4d".to_string(),
                            ),
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
                        Some(SRN::Account(account.id.clone())),
                    )
                    .boxed()
                    .compat(),
            )
            .unwrap();

        let _transfer = rt_01
            .block_on(
                wyre_client
                    .get_transfer(created_transfer.id, Some(SRN::Account(account.id)))
                    .boxed()
                    .compat(),
            )
            .unwrap();
    }
}

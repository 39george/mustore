use fake::faker::internet::en::Username;
use fake::Fake;

use mustore::config::Settings;
use secrecy::ExposeSecret;
use serde::Serialize;

pub struct Banksim {
    pub app_config: Settings,
}

#[derive(Debug)]
pub struct Account {
    pub card_number: String,
    pub password: String,
}

impl Banksim {
    pub async fn create_account(
        &self,
        password: &str,
    ) -> Result<Account, reqwest::Error> {
        let username = Username().fake();
        let client = reqwest::Client::new();
        let endpoint = format!(
            "{}/system/account",
            self.app_config.payments.merchant_api_endpoint
        );
        let response = client
            .post(endpoint)
            .json(&AddAccountRequest {
                username,
                password: password.to_string(),
            })
            .basic_auth(
                "ghashy",
                Some(self.app_config.payments.cashbox_password.expose_secret()),
            )
            .send()
            .await?;
        let response: serde_json::Value = response.json().await?;

        Ok(Account {
            card_number: response
                .get("card_number")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
            password: password.to_string(),
        })
    }
    pub async fn tokens_for_card_number(
        &self,
        card_number: String,
    ) -> Result<Vec<String>, reqwest::Error> {
        let client = reqwest::Client::new();
        let endpoint = format!(
            "{}/system/list_accounts",
            self.app_config.payments.merchant_api_endpoint
        );
        let response = client
            .get(endpoint)
            .basic_auth(
                "ghashy",
                Some(self.app_config.payments.cashbox_password.expose_secret()),
            )
            .send()
            .await?;
        let response: serde_json::Value = response.json().await?;

        let acc = &response
            .get("accounts")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .find(|elem| {
                let candidate =
                    &elem.get("card_number").unwrap().as_str().unwrap();
                candidate.eq(&card_number)
            })
            .unwrap();
        let tokens = acc
            .get("tokens")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|elem| elem.to_string())
            .collect();

        Ok(tokens)
    }
}

#[derive(Serialize)]
struct AddAccountRequest {
    username: String,
    password: String,
}

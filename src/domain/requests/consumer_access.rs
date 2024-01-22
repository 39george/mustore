use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptOffer {
    pub offer_id: i32,
}

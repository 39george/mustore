use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::cornucopia::types::public::Productstatus;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[allow(non_camel_case_types)]
#[schema(example = "moderation")]
pub enum ProductStatus {
    moderation,
    denied,
    active,
    hidden,
    sold,
}

impl From<ProductStatus> for Productstatus {
    fn from(value: ProductStatus) -> Self {
        match value {
            ProductStatus::moderation => Productstatus::moderation,
            ProductStatus::denied => Productstatus::denied,
            ProductStatus::active => Productstatus::active,
            ProductStatus::hidden => Productstatus::hidden,
            ProductStatus::sold => Productstatus::sold,
        }
    }
}


use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RemoveCollectionExternalPluginAdapterV1Args {
    pub key: ExternalPluginAdapterKey,
}
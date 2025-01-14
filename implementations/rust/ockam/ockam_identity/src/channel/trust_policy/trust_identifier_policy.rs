use crate::{IdentityIdentifier, SecureChannelTrustInfo, TrustPolicy};
use ockam_core::Result;
use ockam_core::{async_trait, compat::boxed::Box};

#[derive(Clone)]
pub struct TrustIdentifierPolicy {
    their_identity_id: IdentityIdentifier,
}

impl TrustIdentifierPolicy {
    pub fn new(their_identity_id: IdentityIdentifier) -> Self {
        Self { their_identity_id }
    }
}

#[async_trait]
impl TrustPolicy for TrustIdentifierPolicy {
    async fn check(&mut self, trust_info: &SecureChannelTrustInfo) -> Result<bool> {
        Ok(trust_info.their_identity_id == self.their_identity_id)
    }
}

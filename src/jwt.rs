pub mod errors;
use errors::JwtError;

use crate::{api::v1::contribute::UpdateProofJson, keys};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};
use small_powers_of_tau::sdk::NUM_CEREMONIES;

// Receipt for contributor that coordinator has
// included their contribution
#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub(crate) id_token: IdToken,

    pub witness: [UpdateProofJson; NUM_CEREMONIES],
}

impl Receipt {
    pub fn encode(&self) -> Result<String, JwtError> {
        encode(&Header::default(), self, &crate::keys::KEYS.encoding)
            .map_err(|_| JwtError::TokenCreation)
    }
    pub fn decode(contrib_token: &str) -> Result<Self, JwtError> {
        let token_data =
            decode::<Self>(contrib_token, &keys::KEYS.decoding, &Validation::default())
                .map_err(|_| JwtError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

// This is the JWT token that the coordinator will hand out to contributors
// after they have authenticated through oAUTH
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IdToken {
    pub(crate) sub: String,
    pub nickname: String,
    // The provider whom the client used to login with
    // Example, Google, Ethereum, Facebook
    pub provider: String,
    pub exp: u64,
}

impl IdToken {
    // The sub field is used as a unique identifier
    // For example, see: https://developers.google.com/identity/protocols/oauth2/openid-connect#obtainuserinfo
    // We can use this to identify when a user signs in with the same
    // login and signup
    pub fn unique_identifier(&self) -> &str {
        &self.sub
    }

    pub fn encode(&self) -> Result<String, JwtError> {
        encode(&Header::default(), self, &crate::keys::KEYS.encoding)
            .map_err(|_| JwtError::TokenCreation)
    }
    pub fn decode(contrib_token: &str) -> Result<Self, JwtError> {
        let token_data =
            decode::<Self>(contrib_token, &keys::KEYS.decoding, &Validation::default())
                .map_err(|_| JwtError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
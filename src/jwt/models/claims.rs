use serde::{ Serialize, Deserialize };
use std::time::SystemTime;
use uuid::Uuid;

/// The claims object used for handling JWTs
/// 
/// JWTs consist of three main parts, the header, the body or the 'claims', and the signature.
/// This struct is the representation of the claims of a jwt. This struct has six fields; 
///     sub: subject, which is used to store the user uuid of the user that the token is for
///     iss: issuer, the uuid of the trusted issuer that issued the token
///     role: role, the uuid of the role the user has, used for access control
///     iat: issued at, the time in seconds that the token was issued at
///     nbf: not before, the time in seconds that the token is not valid before
///     exp: expires, the time in seconds that the token is not valid after
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub iss: Uuid,
    pub role: Uuid,
    pub iat: u64,
    pub nbf: u64,
    pub exp: u64,
}

impl Claims {
    pub fn new(subject: Uuid, issuer: Uuid, role: Uuid) -> Self {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        Claims {
            sub: subject.to_owned(),
            iss: issuer.to_owned(),
            role: role.to_owned(),
            iat: now,
            nbf: now,
            exp: now + 3600,
        }
    }
}

use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_pubkey::Pubkey as SolanaPubkey,
    std::str::FromStr,
};

/// Program ID for the HTTP GET program
pub const PROGRAM_ID: &str = "rialoHttpGet1111111111111111111111111111111111";

/// Get the program ID as a Pubkey
pub fn id() -> Pubkey {
    Pubkey::from_str(PROGRAM_ID).unwrap()
}

/// Instruction to make an HTTP GET request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpGetRequest {
    /// The URL to make the GET request to
    pub url: String,
    /// The account to store the response in
    pub response_account: Pubkey,
}

impl HttpGetRequest {
    /// Create a new HTTP GET request instruction
    pub fn new(url: String, response_account: Pubkey) -> Self {
        Self {
            url,
            response_account,
        }
    }

    /// Convert the request into a program instruction
    pub fn into_instruction(self) -> Instruction {
        let accounts = vec![
            AccountMeta::new(self.response_account, false),
        ];

        let data = self.url.into_bytes();

        Instruction {
            program_id: id(),
            accounts,
            data,
        }
    }
}

/// Response data structure for HTTP GET requests
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HttpGetResponse {
    /// The HTTP status code
    pub status_code: u16,
    /// The response body
    pub body: Vec<u8>,
}

impl HttpGetResponse {
    /// Create a new HTTP GET response
    pub fn new(status_code: u16, body: Vec<u8>) -> Self {
        Self {
            status_code,
            body,
        }
    }
}
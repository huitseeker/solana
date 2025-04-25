use {
    super::error::CoreBpfMigrationError,
    reqwest::blocking::Client,
    std::time::Duration,
};

const MAX_RESPONSE_SIZE: usize = 1024; // 1KB max response size
const TIMEOUT_SECONDS: u64 = 1; // 1 second timeout

/// A client for making HTTP requests
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECONDS))
            .build()
            .expect("Failed to create HTTP client");
        Self { client }
    }

    /// Make a GET request to the specified URL and return the response body
    /// as bytes, limited to MAX_RESPONSE_SIZE
    pub fn get(&self, url: &str) -> Result<Vec<u8>, CoreBpfMigrationError> {
        // Validate URL
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(CoreBpfMigrationError::HttpGetInvalidUrl(url.to_string()));
        }

        // Make the request
        let response = self
            .client
            .get(url)
            .send()
            .map_err(|e| CoreBpfMigrationError::HttpGetRequestFailed(e.to_string()))?;

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(CoreBpfMigrationError::HttpGetRequestFailed(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        // Get the response body
        let bytes = response
            .bytes()
            .map_err(|e| CoreBpfMigrationError::HttpGetRequestFailed(e.to_string()))?;

        // Check if the response is too large
        if bytes.len() > MAX_RESPONSE_SIZE {
            return Err(CoreBpfMigrationError::HttpGetResponseTooLarge(
                MAX_RESPONSE_SIZE,
                bytes.len(),
            ));
        }

        Ok(bytes.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{
            io::Write,
            net::TcpListener,
            thread,
        },
    };

    fn start_test_server() -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server_url = format!("http://{}", addr);

        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let response = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello";
                stream.write_all(response.as_bytes()).unwrap();
            }
        });

        (server_url, handle)
    }

    #[test]
    fn test_http_get_success() {
        let (server_url, _handle) = start_test_server();
        let client = HttpClient::new();
        let response = client.get(&server_url).unwrap();
        assert_eq!(response, b"Hello");
    }

    #[test]
    fn test_http_get_invalid_url() {
        let client = HttpClient::new();
        let result = client.get("invalid-url");
        assert!(matches!(
            result,
            Err(CoreBpfMigrationError::HttpGetInvalidUrl(_))
        ));
    }

    #[test]
    fn test_http_get_timeout() {
        let client = HttpClient::new();
        let result = client.get("http://10.255.255.255");
        assert!(matches!(
            result,
            Err(CoreBpfMigrationError::HttpGetRequestFailed(_))
        ));
    }
}
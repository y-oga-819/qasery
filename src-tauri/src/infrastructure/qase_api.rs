// Qase API Client implementation

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qase_client_initialization() {
        // QaseClientが正しく初期化できることをテスト
        let api_token = "test_api_token";
        let client = QaseClient::new(api_token);

        assert!(client.is_ok());
    }
}

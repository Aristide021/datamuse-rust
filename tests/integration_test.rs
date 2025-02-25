use datamuse_rust::DatamuseClient;

#[tokio::test]
async fn test_words_endpoint() {
    let client = DatamuseClient::new();
    let word_list = client.words().means_like("ocean").call().await.unwrap();
    assert!(!word_list.is_empty());
}

#[tokio::test]
async fn test_sug_endpoint() {
    let client = DatamuseClient::new();
    let suggestion_list = client.sug().hint_string("auto").call().await.unwrap();
    assert!(!suggestion_list.is_empty());
}

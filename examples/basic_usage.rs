use datamuse_rust::DatamuseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Basic usage of the Datamuse API wrapper\n");

    let client = DatamuseClient::new();

    // Find words that mean something like "example"
    println!("Finding words that mean something like 'example':");
    let words = client.words().means_like("example").max(5).call().await?;

    // Print each word with its score
    for word in words {
        println!("- {} (score: {})", word.word, word.score);
    }

    // Get suggestions for a partial word
    println!("\nGetting suggestions for 'auto':");
    let suggestions = client.sug().hint_string("auto").max(5).call().await?;

    // Print each suggestion with its score
    for suggestion in suggestions {
        println!("- {} (score: {})", suggestion.word, suggestion.score);
    }

    Ok(())
}

# datamuse-rust

A comprehensive, production-grade Rust wrapper for the [Datamuse API](https://www.datamuse.com/api/), providing an ergonomic interface for word finding operations.

## Features

- Full support for all Datamuse API parameters and endpoints
- Asynchronous API with Tokio runtime
- Type-safe parameter handling
- Comprehensive error handling
- Detailed documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
datamuse-rust = "0.1.0"
```

## Usage

### Basic Example

```rust
use datamuse_rust::DatamuseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DatamuseClient::new();

    // Find words that mean something like "example"
    let words = client.words().means_like("example").max(5).call().await?;
    
    for word in words {
        println!("- {} (score: {})", word.word, word.score);
    }

    Ok(())
}
```

### Advanced Example

```rust
use datamuse_rust::DatamuseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DatamuseClient::new();

    // Find words that mean 'good' and sound like 'great'
    let similar = client
        .words()
        .means_like("good")
        .sounds_like("great")
        .max(5)
        .call()
        .await?;

    // Find words with definitions
    let words = client
        .words()
        .means_like("big")
        .meta_data("d")  // request definitions
        .max(3)
        .call()
        .await?;

    // Get autocomplete suggestions
    let suggestions = client
        .sug()
        .hint_string("eleph")
        .max(5)
        .call()
        .await?;

    Ok(())
}
```

For more examples, see the [examples directory](examples/).

## API Documentation

### Endpoints

#### /words
- `means_like(term)`: Find words with a meaning similar to the specified term
- `sounds_like(term)`: Find words that sound similar to the specified term
- `spelled_like(term)`: Find words that are spelled similarly
- `related_synonyms(term)`: Find synonyms
- `related_triggers(term)`: Find "triggers" (words that are statistically associated)
- `related_antonyms(term)`: Find antonyms
- And more related word parameters (rel_jja, rel_jjb, etc.)

#### /sug
- `hint_string(term)`: Get autocomplete suggestions for a partial word
- `max(n)`: Limit the number of results

### Parameters
- `meta_data(flags)`: Request additional word metadata
- `max(n)`: Limit the number of results
- `left_context(text)`: Specify left context for better suggestions
- `right_context(text)`: Specify right context for better suggestions

For full API documentation, see the [Datamuse API documentation](https://www.datamuse.com/api/).

## Acknowledgments

This library is a wrapper around the excellent [Datamuse API](https://www.datamuse.com/api/), a word-finding query engine created by [Datamuse](https://www.datamuse.com/). We are grateful to Datamuse for providing this valuable service to the developer community. Please review their [terms of service](https://www.datamuse.com/api/) for usage guidelines and limitations.

## License

This project is licensed under the MIT License - see below for details:

```
MIT License

Copyright (c) 2025 Sheldon Aristide

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

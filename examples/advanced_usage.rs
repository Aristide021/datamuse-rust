use datamuse_rust::DatamuseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Advanced usage of the Datamuse API wrapper\n");

    let client = DatamuseClient::new();

    // 1. Find words that mean something like "good" and sound like "great"
    println!("1. Words that mean 'good' and sound like 'great':");
    let similar = client
        .words()
        .means_like("good")
        .sounds_like("great")
        .max(5)
        .call()
        .await?;

    for word in similar {
        println!("- {} (score: {})", word.word, word.score);
    }

    // 2. Find words that rhyme with "bright"
    println!("\n2. Words that rhyme with 'bright':");
    let rhyming = client
        .words()
        .sounds_like("bright")
        .max(5)
        .call()
        .await?;

    for word in rhyming {
        println!("- {} (score: {})", word.word, word.score);
    }

    // 3. Find words meaning similar to "big" with definitions
    println!("\n3. Words similar to 'big' with definitions:");
    let words = client
        .words()
        .means_like("big")
        .meta_data("d")  // request definitions
        .max(3)
        .call()
        .await?;

    for word in words {
        print!("- {} (score: {})", word.word, word.score);
        if let Some(defs) = word.defs {
            println!("\n  Definitions:");
            for def in defs {
                println!("  * {}", def);
            }
        } else {
            println!();
        }
    }

    // 4. Get autocomplete suggestions for a partial word
    println!("\n4. Autocomplete suggestions for 'eleph':");
    let suggestions = client
        .sug()
        .hint_string("eleph")
        .max(5)
        .call()
        .await?;

    for word in suggestions {
        println!("- {} (score: {})", word.word, word.score);
    }

    Ok(())
}

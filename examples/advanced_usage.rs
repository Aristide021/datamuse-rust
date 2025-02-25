use datamuse_rust::DatamuseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Advanced usage of the Datamuse API wrapper\n");

    let client = DatamuseClient::new();

    // 1. Find rhyming words with frequency data
    println!("1. Words that rhyme with 'bright' (with frequency data):");
    let rhyming = client
        .words()
        .related_rhymes("bright")
        .meta_data("f")  // request frequency data
        .max(5)
        .call()
        .await?;

    for word in rhyming {
        if let Some(freq) = word.frequency {
            println!("- {} (score: {}, frequency: {:.2})", word.word, word.score, freq);
        } else {
            println!("- {} (score: {})", word.word, word.score);
        }
    }

    // 2. Find words starting with 'ele' and get their definitions
    println!("\n2. Words starting with 'ele' and their definitions:");
    let starting_with = client
        .words()
        .starts_with("ele")
        .meta_data("d")  // request definitions
        .max(3)
        .call()
        .await?;

    for word in starting_with {
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

    // 3. Find words that sound like "great" and mean "good" (combined query)
    println!("\n3. Words that sound like 'great' and mean 'good':");
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

    // 4. Find near rhymes with metadata
    println!("\n4. Near rhymes for 'blue' with metadata (frequency and definitions):");
    let near_rhymes = client
        .words()
        .related_near_rhymes("blue")
        .meta_data("df")  // request both definitions and frequency
        .max(3)
        .call()
        .await?;

    for word in near_rhymes {
        print!("- {} (score: {})", word.word, word.score);
        if let Some(freq) = word.frequency {
            print!(", frequency: {:.2}", freq);
        }
        if let Some(defs) = word.defs {
            println!("\n  Definitions:");
            for def in defs {
                println!("  * {}", def);
            }
        } else {
            println!();
        }
    }

    Ok(())
}

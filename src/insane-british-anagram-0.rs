//
// insane-british-anagram-0.rs - Find words that have valid anagrams
//                               Words sourced from Debian's british-english-insane dictionary
//
// heater - 2019-07-29
// 

#![allow(non_snake_case)]

use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::collections::HashMap;

fn readInsaneBritishDictionary(mut dictionary: &mut Vec<u8>) -> std::io::Result<()> {
    let mut file = File::open("/usr/share/dict/british-english-insane")?;
    file.read_to_end(&mut dictionary)?;
    return Ok(());
}

fn primeHash (slice: &[u8]) -> u64 {
    // One prime number for each lower case letter of the alphabet
    let primes: [u64; 26] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101];

    let mut hash : u64 = 1;
    for c in slice {
        let index = (c -  97) as usize;  
        hash = hash.wrapping_mul(primes[index]);
    }
    return hash;
}

fn isLowerCase (c : &u8) -> bool {
    if (*c < 'a' as u8) || (*c > 'z' as u8) {
        return false;
    } else {
        return true;
    }
}

fn main() {
    let stdout = io::stdout();
    let mut stdoutHandle = stdout.lock();

    // Map container for sets of anagrams 
    // An anagram set is simply a vector of pointers to word strings
    let mut anagramMap: HashMap<u64, Vec<String>> = HashMap::new();

    // An ordered index of anagram set keys 
    let mut index: Vec<u64> = Vec::new();

    let mut dictionary = Vec::new();
    match readInsaneBritishDictionary(&mut dictionary) {
        Ok(()) => {
            let mut wordIndex = 0;
            let mut characterIndex = 0;
            let mut reject = false;
            for c in &dictionary  {
                if isLowerCase(c) {
                    // We are scanning a valid word
                    characterIndex = characterIndex + 1;
                } else if *c == '\n' as u8 {
                    // We have hit the end of a word, use the word if it's valid
                    if !reject {
                        // Do we have a word with this key (potential anagram)?
                        let word = &dictionary[wordIndex .. characterIndex];
                        let hash = primeHash(word);
                        let string = String::from_utf8_lossy(word).to_string();
                        match anagramMap.get_mut(&hash) {
                            Some(anagramSet) => {
                                // Found: Append it to the existing anagram set
                                anagramSet.push(string);
                            },
                            None => {
                                // Not found: Add it to the map as start of new anagram set.
                                let mut anagramSet: Vec<String> = Vec::new();
                                
                                anagramSet.push(string);
                                anagramMap.insert(hash, anagramSet);

                                // And add the new anagram set to index
                                index.push(hash);
                            }
                        }
                    }
                    characterIndex = characterIndex + 1;
                    wordIndex = characterIndex;
                    reject = false;
                } else {
                    // Invalid character
                    reject = true;
                    characterIndex = characterIndex + 1;
                }
            }

            let mut output: String = "".to_string();
            for hash in index {
                match anagramMap.get(&hash) {
                    Some(anagramSet) => {
                        if anagramSet.len() > 1 {
                            output = output + &anagramSet[0];
                            let mut separator = ": ";
                            for word in &anagramSet[1..] {
                                output = output + &separator;
                                output = output + &word;
                                separator = ", ";
                            }
                            output = output + "\n";
                        }
                    },
                    _ => (),
                }
            }
            match stdoutHandle.write_all(output.as_bytes()) {
                Ok(()) => (),
                Err(e) => println!("Error writing reult {}", e),
            }
        },
        Err(e) => {
            println!("Error reading dictionary: {}", e);
        }
    }
}
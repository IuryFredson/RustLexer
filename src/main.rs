use std::io;

// Analyze the input string and extract lexical elements
fn analyze(input: &str) -> Result<(usize, &str, &str), Option<usize>> {
    // Create a peekable iterator over characters with their indices
    let mut characters = input.char_indices().peekable();

    // Skip leading whitespaces and special characters
    while let Some((_, c)) = characters.peek() {
        if c.is_whitespace() || *c == '🦀' {
            characters.next();
        } else {
            break;
        }
    }

    // Determine the start of the meaningful content
    let start = match characters.peek() {
        Some(&(index, _)) => index,
        None => return Err(None),
    };

    // Remaining text after skipping whitespaces
    let remaining = &input[start..];
    
    // Extract the lexical token
    let (pos, content, new_remaining) = match characters.next() {
        Some((index, c)) if c.is_ascii_digit() => {
            // Parse consecutive digits
            let mut end = index + c.len_utf8();
            while let Some(&(index, c)) = characters.peek() {
                if c.is_ascii_digit() {
                    end = index + c.len_utf8();
                    characters.next();
                } else {
                    break;
                }
            }
            (start, &input[start..end], &input[end..])
        }
        // Recognize operators and special symbols
        Some((index, c)) if c == '+' || c == '-' || c == '*' || c == '/' || c == '🐧' => {
            let end = index + c.len_utf8();
            (start, &input[start..end], &input[end..])
        }
        // Handle unexpected characters
        Some((index, _)) => {
            return Err(Some(index + 1));
        }
        None => return Err(None),
    };

    Ok((pos, content, new_remaining))
}

fn main() {
    // Prompt for user input
    let mut user_input = String::new();
    println!("Enter an expression to analyze:");
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    let user_input = user_input.trim();

    println!("Analyzing: \"{}\"", user_input);
    
    // Track parsing process
    let mut remaining_text = user_input;
    let mut total_position = 1;
    
    // Process input until fully parsed
    while !remaining_text.is_empty() {
        match analyze(remaining_text) {
            Ok((pos, content, new_remaining)) => {
                // Calculate exact token position
                let updated_position = total_position + remaining_text[..pos].chars().count();
                println!("Token found: (Position: {}, Content: \"{}\")", updated_position, content);
                
                // Update tracking variables
                total_position = updated_position + content.chars().count();
                remaining_text = new_remaining;
            }
            Err(Some(pos)) => {
                // Report parsing error with location
                let updated_position = total_position + remaining_text[..pos].chars().count() - 1;
                println!("Error found at position {}", updated_position);
                break;
            }
            Err(None) => {
                // No more tokens to parse
                println!("No lexical elements remaining.");
                break;
            }
        }
    }
}
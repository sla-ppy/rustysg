// Input text
// <p> This is my first post, {{ content.html }} like and subscribe! </p>

// After tokenization, output!
/*
<p> This is my first post,
{{
content.html
}}
like and subscribe! </p>
*/

/*
template   = *(text / expression)
text       = *CHAR
expression = "{{" *CHAR "}}"
 */
use std::fmt::format;

#[derive(Debug, PartialEq)]
enum Token {
    Text(String),
    OpeningBrace,
    ClosingBrace
}

fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut accumulator = String::new();
    let mut skip_next = false;

    for (i, c) in str.chars().enumerate() {
        let c_next = str.chars().nth(i+1);
        if skip_next {
            skip_next = false;
            continue;
        }

        if c == '{' && c_next == Some('{') {
            if !accumulator.is_empty() {
                tokens.push(Token::Text(accumulator.clone()));
            }
            tokens.push(Token::OpeningBrace);
            skip_next = true;
            accumulator.clear();
        } else if c == '}' && c_next == Some('}')  {
            if !accumulator.is_empty() {
                tokens.push(Token::Text(accumulator.clone()));
            }
            tokens.push(Token::ClosingBrace);
            skip_next = true;
            accumulator.clear();
        } else {
            skip_next = false;
            accumulator.push(c);
        }
    }

    if !accumulator.is_empty() {
        tokens.push(Token::Text(accumulator));
    } else {
        // nothing
    }

    println!("{:?}", tokens);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test() {
    }

    #[test]
    fn test_tokenize() {
        tokenize("<p> This is my first post, {{ content.html }} like and subscribe! </p>");
        assert_eq!(tokenize("{{{{"), vec![Token::OpeningBrace, Token::OpeningBrace]);
        assert_eq!(tokenize("äÄ$Đß&@÷ŧ÷Äß÷Đß"), vec![Token::Text("äÄ$Đß&@÷ŧ÷Äß÷Đß".to_string())]);
        //tokenize("");
    }
}
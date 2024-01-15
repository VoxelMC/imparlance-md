// #![allow(dead_code)]
//
// TODO: DO this with Tokenizer, THEN parse to HTML,
// But it's a great start!
//

// struct CustomToken {
//     handler: fn(&mut self) -> String,
// }

struct Parser {
    pos: usize,
    input: String,
    custom_tokens: Option<Vec<fn() -> String>>,
}

// trait Parser {
//     fn handle_title(&mut self) -> String;
// }

impl Parser {
    fn start_parse_lines(&mut self) -> String {
        let mut result = String::new();

        loop {
            self.consume_whitespace();
            if self.end_of_line() {
                break;
            }
            result.push_str(&self.parse_line())
        }

        result
    }

    fn parse_line(&mut self) -> String {
        match self.next_char() {
            '#' => self.title_parse(),
            '-' => {
                if char::is_whitespace(self.input[self.pos + 1..].chars().next().unwrap()) {
                    self.list_parse()
                } else {
                    self.text_parse()
                }
            }
            _ => self.text_parse(),
        }
    }

    fn title_parse(&mut self) -> String {
        let hash = self.consume_while(|ch| ch == '#');
        self.consume_whitespace();
        let text = self.text_parse();

        create_html_element(
            format!("h{}", if hash.len() <= 6 { hash.len() } else { 6 }),
            text,
        )
    }

    fn text_parse(&mut self) -> String {
        self.consume_while(|ch| !is_newline(ch))
    }

    fn list_parse(&mut self) -> String {
        self.consume_char();
        self.consume_whitespace();
        let text = self.text_parse();
        create_html_element("li".to_string(), text)
    }

    fn end_of_line(&self) -> bool {
        self.pos >= self.input.len()
    }

    // fn starts_with(&self, token: &str) -> bool {
    //     self.input[self.pos..].starts_with(token)
    // }

    fn next_char(&self) -> char {
        self.input[self.pos..]
            .chars()
            .next()
            .expect("Could not advance one character forward!")
    }

    fn consume_char(&mut self) -> char {
        let mut iterator = self.input[self.pos..].char_indices();
        let (_, current_char) = iterator
            .next()
            .expect("Could not advance one character forward in consumption!");
        let (next_pos, _) = iterator.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        current_char
    }

    fn consume_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.end_of_line() && predicate(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}

fn create_html_element(tag_name: String, text: String) -> String {
    format!("<{}>{}</{}>", tag_name, text, tag_name)
}

fn is_newline(ch: char) -> bool {
    ch == '\n'
}

pub fn parse(markdown_text: String) -> String {
    Parser {
        pos: 0,
        input: markdown_text,
        custom_tokens: None,
    }
    .start_parse_lines()
}

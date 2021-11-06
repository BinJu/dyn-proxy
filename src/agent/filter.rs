use std::ops::Range;

/// The filter for handling the <div> tag. Currently the only way we handle the matched <div> is to remove it
///
pub struct ContentFilter {
    div_props: Vec<String>
}

impl ContentFilter {
    /// Return a ContentFilter inatance
    /// The parameter props is for identifying the div tag. e.g. <div id="123" ... >something</div>
    /// the prop should be: `id="123"`. if you pass it as a command line parameter, make sure you
    /// single quote it in the commandline.
    /// Multi tag properties for removing multiple divs.
    #[inline]
    pub fn new(props: Vec<String>) -> ContentFilter{ ContentFilter{div_props: props}}

    /// Filter the text, remove the div tag.
    pub fn filter(&self, src: String) -> String {
        if !src.contains("<html") {
            String::from(src)
        } else {
            let mut content = src.to_owned();
            for div_prop in &self.div_props {
                content = Self::remove_div(&content, &*div_prop)
            }
            content
        }
    }

    fn remove_div(src: &String, div_props: &str) -> String {
        let mut div_tokenizer = DivTokenizer::new(src, div_props);
        match div_tokenizer.locate() {
            Ok(range) => {
                let mut cloned = src.clone();
                cloned.replace_range(range, "");
                cloned
                
            },
            Err(msg)=> { println!("[Warning] {}", msg); String::from(src)}
        }
    }
}

struct DivTokenizer <'a> {
    div_props: &'a str,
    at: usize,
    cursor: usize,
    src: &'a str,
    last_token: Option<usize>
}

impl<'a> DivTokenizer<'a> {
    fn new(src: &'a str, div_props: &'a str) -> DivTokenizer<'a> { DivTokenizer{div_props, src, at: 0, cursor: 0, last_token: None} }
    fn locate(&mut self) -> std::result::Result<Range<usize>, String> {
        let div_tag = format!("{} {}", "<div", self.div_props);
        match self.src.find(&*div_tag) {
            Some(pos) => { self.at = pos; self.cursor = self.at + 1 },
            None => { return Err(format!("Can not locate the <div {}", self.div_props));}
        };

        let mut stack = 1;
        while let Some(token) = self.next_token() {
            if token == "<div" {
               stack += 1; 
            } else if token =="</div" {
                stack -= 1;
            }
            if stack == 0 {
                break;
            }
        }
        if stack == 0 {
            Ok(Range{start: self.at, end: self.cursor})
        } else {
           Err(format!("Html tag div is not closed. the stack value: {}", stack)) 
        }
    }
    fn next_token(&mut self) -> Option<&str> {
        let mut max_search_len = self.at + 20_000;
        if self.src.len() < max_search_len { max_search_len = self.src.len() }

        for i in self.cursor .. max_search_len {
            match &self.src[i..i+1] {
                ">" | " " => {
                    if let Some(last) = self.last_token {
                        let token = &self.src[last..i];
                        self.cursor += 1;
                        self.last_token = None;
                        //println!("EEEE current token : {}| i={}, cursor={} raw: {}", token, i, self.cursor, &self.src[i-10..i]);
                        return Some(token)
                    }
                },
                "<" => {
                    self.last_token = Some(i);
                },
                _ => {}
            }
            self.cursor += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_next_token() {
        let src = r#"this is a test <div class="abc">something</div><div class="ee"><div id="inner1"><div id="inner2">text</div></div></div>something else"#;
        let mut token = DivTokenizer::new(src, r#"class="ee""#);
        let next = token.next_token();
        assert!(next.is_some());
        assert_eq!(next.unwrap(), "<div");
        let next = token.next_token();
        assert!(next.is_some());
        assert_eq!(next.unwrap(), "</div");

    }
    #[test]
    fn test_tokenizer_locate() {
        let src = r#"this is a test <div class="abc">something</div><div class="ee"><div id="inner1"><div id="inner2">text</div></div></div>something else"#;
        let mut token = DivTokenizer::new(src, r#"class="ee""#);
        let pos = token.locate();
        if let Err(msg) = pos {
            assert_eq!(msg, "");
            panic!("token.locate() shoud return Ok");
        } else {
            let pos = pos.unwrap();
            assert_eq!(pos.start, 47);
            assert_eq!(pos.end, 119);
            let mut text = String::from(src);
            text.replace_range(pos, "");
            assert_eq!(text, r#"this is a test <div class="abc">something</div>something else"#)
        }
    }
}

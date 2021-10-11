use crate::symbol;
use std::error::Error;

pub struct Body<'a> {
    body: &'a [u8],
    pub start_index: usize,

    is_text: bool,
    text_buffer: Vec<u8>,
    pub exist_text: bool,
}

impl<'a> Body<'a> {
    //! A impl that holds the contents used during analysis.
    //! Character analysis is performed by popping the character from the beginning.
    //! Holds various other information.

    /// Create a new this instance.
    ///
    /// ## Arguments
    ///
    /// - `body` - target json data.
    pub fn new(body: &'a str) -> Self {
        Self {
            body: body.as_bytes(),
            start_index: 0,
            is_text: false,
            text_buffer: Vec::new(),
            exist_text: false,
        }
    }

    /// Get json data character by byte from the beginning.
    ///
    /// If there are letters or numbers enclosed in double quotation marks, store them in text_buffer.
    /// Check exist_text for how to check the buffer.
    /// Call get_text() to get this.
    pub fn pop(&mut self) -> Option<&u8> {
        if self.body.len() <= self.start_index {
            None
        } else {
            let target = &self.body[self.start_index];
            self.start_index += 1;

            // Buffers a string or number string.
            if self.is_text {
                self.push_text(&target);
                self.pop()
            } else if symbol::is_double_quotation(&target) {
                self.exist_text = true;

                // If enclosed in double quotes, it is a string.
                self.is_text = !self.is_text;
                self.pop()
            } else if symbol::is_number(&target) {
                self.exist_text = true;

                // number.
                // Note: Numbers in double quotes are not counted.
                self.push_text(&target);
                self.pop()
            } else {
                if self.exist_text {
                    self.exist_text = false;
                }

                Some(target)
            }
        }
    }

    /// Push char in buffer.
    fn push_text(&mut self, tx: &u8) {
        self.text_buffer.push(*tx);
    }

    /// Get the buffer and clear buffer.
    pub fn get_text(&mut self) -> Result<String, Box<dyn Error>> {
        let result = String::from_utf8(self.text_buffer.clone())?;

        // clear buffer
        self.text_buffer = Vec::new();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::body::Body;

    #[test]
    fn pop_works() {
        let sample_txt = "aiueonya";
        let text_length = sample_txt.len();
        let sample_txt_bytes = sample_txt.as_bytes();

        let mut body = Body::new(sample_txt);

        for byte in sample_txt_bytes {
            assert_eq!(byte, body.pop().unwrap());
        }

        assert_eq!(text_length, body.start_index);
    }

    #[test]
    fn buffer_works() {
        let sample_txt = "hogehoge \"fuga\"foo";

        let mut body = Body::new(sample_txt);

        loop {
            let result = body.pop();

            match result {
                Some(_) => {
                    if body.exist_text {
                        let text = body.get_text().unwrap();
                        assert_eq!("fuga", text);
                    }
                }
                None => break,
            }
        }
    }
}

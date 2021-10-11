pub struct Body<'a> {
    body: &'a [u8],
    pub start_index: usize,
}

impl<'a> Body<'a> {
    //! A impl that holds the contents used during analysis.
    //! Character analysis is performed by popping the character from the beginning.
    //! Holds various other information.

    pub fn new(body: &'a str) -> Self {
        Self {
            body: body.as_bytes(),
            start_index: 0,
        }
    }

    pub fn pop(&mut self) -> &u8 {
        let target = &self.body[self.start_index];
        self.start_index += 1;

        target
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
            assert_eq!(byte, body.pop());
        }

        assert_eq!(text_length, body.start_index);
    }
}

pub(crate) struct Scanner<'a> {
    source: &'a String,
}

impl Scanner<'_> {
    pub fn new(source: &String) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> anyhow::Result<()> {
        println!("Source: {}", self.source);
        Ok(())
    }
}

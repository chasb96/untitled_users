pub enum Period {
    All,
}

impl Period {
    pub fn as_where_clause(&self) -> &'static str {
        match self {
            Period::All => "1=1",
        }
    }
}
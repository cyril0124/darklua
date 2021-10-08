use crate::nodes::{Expression, Identifier, Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableFieldEntry {
    field: Identifier,
    value: Expression,
    /// The token for the `=` operator symbol.
    token: Option<Token>,
}

impl TableFieldEntry {
    pub fn new<I: Into<Identifier>, E: Into<Expression>>(field: I, value: E) -> Self {
        Self {
            field: field.into(),
            value: value.into(),
            token: None,
        }
    }

    pub fn with_token(mut self, token: Token) -> Self {
        self.token = Some(token);
        self
    }

    #[inline]
    pub fn set_token(&mut self, token: Token) {
        self.token = Some(token);
    }

    #[inline]
    pub fn get_field(&self) -> &Identifier {
        &self.field
    }

    #[inline]
    pub fn mutate_field(&mut self) -> &mut Identifier {
        &mut self.field
    }

    #[inline]
    pub fn get_value(&self) -> &Expression {
        &self.value
    }

    #[inline]
    pub fn mutate_value(&mut self) -> &mut Expression {
        &mut self.value
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableIndexEntryTokens {
    pub opening_bracket: Token,
    pub closing_bracket: Token,
    pub equal: Token,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableIndexEntry {
    key: Expression,
    value: Expression,
    tokens: Option<Box<TableIndexEntryTokens>>,
}

impl TableIndexEntry {
    pub fn new<T: Into<Expression>, U: Into<Expression>>(key: T, value: U) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            tokens: None,
        }
    }

    pub fn with_tokens(mut self, tokens: TableIndexEntryTokens) -> Self {
        self.tokens = Some(tokens.into());
        self
    }

    #[inline]
    pub fn set_tokens(&mut self, tokens: TableIndexEntryTokens) {
        self.tokens = Some(tokens.into());
    }

    #[inline]
    pub fn get_key(&self) -> &Expression {
        &self.key
    }

    #[inline]
    pub fn mutate_key(&mut self) -> &mut Expression {
        &mut self.key
    }

    #[inline]
    pub fn get_value(&self) -> &Expression {
        &self.value
    }

    #[inline]
    pub fn mutate_value(&mut self) -> &mut Expression {
        &mut self.value
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TableEntry {
    Field(TableFieldEntry),
    Index(TableIndexEntry),
    Value(Expression),
}

impl From<TableFieldEntry> for TableEntry {
    fn from(entry: TableFieldEntry) -> Self {
        Self::Field(entry)
    }
}

impl From<TableIndexEntry> for TableEntry {
    fn from(entry: TableIndexEntry) -> Self {
        Self::Index(entry)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableTokens {
    pub opening_brace: Token,
    pub closing_brace: Token,
    pub separators: Vec<Token>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableExpression {
    entries: Vec<TableEntry>,
    tokens: Option<TableTokens>,
}

impl TableExpression {
    pub fn new(entries: Vec<TableEntry>) -> Self {
        Self {
            entries,
            tokens: None,
        }
    }

    pub fn with_tokens(mut self, tokens: TableTokens) -> Self {
        self.tokens = Some(tokens);
        self
    }

    #[inline]
    pub fn set_tokens(&mut self, tokens: TableTokens) {
        self.tokens = Some(tokens);
    }

    #[inline]
    pub fn get_entries(&self) -> &Vec<TableEntry> {
        &self.entries
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[inline]
    pub fn mutate_entries(&mut self) -> &mut Vec<TableEntry> {
        &mut self.entries
    }

    pub fn append_entry<T: Into<TableEntry>>(mut self, entry: T) -> Self {
        self.entries.push(entry.into());
        self
    }

    pub fn append_field<S: Into<Identifier>, E: Into<Expression>>(
        mut self,
        key: S,
        value: E,
    ) -> Self {
        self.entries.push(TableFieldEntry::new(key, value).into());
        self
    }

    pub fn append_index<T: Into<Expression>, U: Into<Expression>>(
        mut self,
        key: T,
        value: U,
    ) -> Self {
        self.entries
            .push(TableIndexEntry::new(key.into(), value.into()).into());
        self
    }

    pub fn append_array_value<E: Into<Expression>>(mut self, value: E) -> Self {
        self.entries.push(TableEntry::Value(value.into()));
        self
    }
}

impl Default for TableExpression {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

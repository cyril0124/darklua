use crate::nodes::{Block, Identifier, Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalFunctionTokens {
    pub local: Token,
    pub function: Token,
    pub opening_parenthese: Token,
    pub closing_parenthese: Token,
    pub end: Token,
    pub parameter_commas: Vec<Token>,
    pub variable_arguments: Option<Token>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalFunctionStatement {
    identifier: Identifier,
    block: Block,
    parameters: Vec<Identifier>,
    is_variadic: bool,
    tokens: Option<Box<LocalFunctionTokens>>,
}

impl LocalFunctionStatement {
    pub fn new(
        identifier: Identifier,
        block: Block,
        parameters: Vec<Identifier>,
        is_variadic: bool,
    ) -> Self {
        Self {
            identifier,
            block,
            parameters,
            is_variadic,
            tokens: None,
        }
    }

    pub fn from_name<S: Into<Identifier>, B: Into<Block>>(identifier: S, block: B) -> Self {
        Self {
            identifier: identifier.into(),
            block: block.into(),
            parameters: Vec::new(),
            is_variadic: false,
            tokens: None,
        }
    }

    pub fn with_tokens(mut self, tokens: LocalFunctionTokens) -> Self {
        self.tokens = Some(tokens.into());
        self
    }

    #[inline]
    pub fn set_tokens(&mut self, tokens: LocalFunctionTokens) {
        self.tokens = Some(tokens.into());
    }

    pub fn with_parameter<S: Into<Identifier>>(mut self, parameter: S) -> Self {
        self.parameters.push(parameter.into());
        self
    }

    pub fn variadic(mut self) -> Self {
        self.is_variadic = true;
        self
    }

    #[inline]
    pub fn mutate_parameters(&mut self) -> &mut Vec<Identifier> {
        &mut self.parameters
    }

    #[inline]
    pub fn mutate_block(&mut self) -> &mut Block {
        &mut self.block
    }

    #[inline]
    pub fn mutate_identifier(&mut self) -> &mut Identifier {
        &mut self.identifier
    }

    #[inline]
    pub fn get_block(&self) -> &Block {
        &self.block
    }

    #[inline]
    pub fn get_parameters(&self) -> &Vec<Identifier> {
        &self.parameters
    }

    #[inline]
    pub fn get_identifier(&self) -> &Identifier {
        &self.identifier
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        self.identifier.get_name()
    }

    #[inline]
    pub fn has_parameter(&self, name: &str) -> bool {
        self.parameters
            .iter()
            .any(|parameter| parameter.get_name() == name)
    }

    #[inline]
    pub fn is_variadic(&self) -> bool {
        self.is_variadic
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_parameter_is_true_when_single_param_matches() {
        let func = LocalFunctionStatement::from_name("foo", Block::default()).with_parameter("bar");

        assert!(func.has_parameter("bar"));
    }

    #[test]
    fn has_parameter_is_true_when_at_least_one_param_matches() {
        let func = LocalFunctionStatement::from_name("foo", Block::default())
            .with_parameter("bar")
            .with_parameter("baz");

        assert!(func.has_parameter("baz"));
    }

    #[test]
    fn has_parameter_is_false_when_none_matches() {
        let func = LocalFunctionStatement::from_name("foo", Block::default())
            .with_parameter("bar")
            .with_parameter("baz");

        assert!(!func.has_parameter("foo"));
    }
}

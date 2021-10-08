use crate::nodes::{Expression, Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParentheseTokens {
    pub left_parenthese: Token,
    pub right_parenthese: Token,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParentheseExpression {
    expression: Expression,
    tokens: Option<ParentheseTokens>,
}

impl ParentheseExpression {
    pub fn new<E: Into<Expression>>(expression: E) -> Self {
        Self {
            expression: expression.into(),
            tokens: None,
        }
    }

    #[inline]
    pub fn inner_expression(&self) -> &Expression {
        &self.expression
    }

    #[inline]
    pub fn into_inner_expression(self) -> Expression {
        self.expression
    }

    #[inline]
    pub fn mutate_inner_expression(&mut self) -> &mut Expression {
        &mut self.expression
    }

    pub fn with_tokens(mut self, tokens: ParentheseTokens) -> Self {
        self.tokens = Some(tokens);
        self
    }

    #[inline]
    pub fn set_tokens(&mut self, tokens: ParentheseTokens) {
        self.tokens = Some(tokens);
    }
}

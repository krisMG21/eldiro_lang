use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug)]
pub struct VariableDef(SyntaxNode);

impl VariableDef {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn value(&self) -> Option<SytnaxNode> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Literal(Literal),
    Paren(ParenExpr),
    Prefix(PrefixExpr),
    VariableRef(VariableRef),
}

#[derive(Debug)]
pub struct BinaryExpr(SyntaxNode);

#[derive(Debug)]
pub struct Literal(SyntaxNode);

#[derive(Debug)]
pub struct ParenExpr(SyntaxNode);

#[derive(Debug)]
pub struct PrefixExpr(SyntaxNode);

#[derive(Debug)]
pub struct VariableRef(SyntaxNode);

impl Expr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::InfixExpr => Expr::Binary(BinaryExpr(node)),
            SyntaxKind::Literal => Expr::Literal(Literal(node)),
            SyntaxKind::ParenExpr => Expr::Paren(ParenExpr(node)),
            SyntaxKind::PrefixExpr => Expr::Prefix(PrefixExpr(node)),
            SyntaxKind::VariableRef => Expr::VariableRef(VariableRef(node)),
            _ => return None,
        };

        Some(result)
    }
}

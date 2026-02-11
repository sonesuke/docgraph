use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub match_clause: MatchClause,
    pub where_clause: Option<WhereClause>,
    pub return_clause: ReturnClause,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchClause {
    pub patterns: Vec<PatternPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternPart {
    pub chains: Vec<PatternChain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternChain {
    Node(NodePattern),
    Relationship(RelationshipPattern, NodePattern),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePattern {
    pub variable: Option<String>,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipPattern {
    pub variable: Option<String>,
    pub rel_type: Option<String>,
    pub range: Option<Range>,
    pub direction: Direction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Range {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhereClause {
    pub expression: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Or(Vec<Expression>),
    And(Vec<Expression>),
    Comparison(Comparison),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparison {
    pub left: PropertyOrVariable,
    pub operator: Option<ComparisonOperator>,
    pub right: Option<Term>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyOrVariable {
    pub variable: String,
    pub property: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Term {
    Literal(Literal),
    PropertyOrVariable(PropertyOrVariable),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Number(i64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnClause {
    pub items: Vec<ReturnItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnItem {
    pub expression: Expression,
    pub alias: Option<String>,
}

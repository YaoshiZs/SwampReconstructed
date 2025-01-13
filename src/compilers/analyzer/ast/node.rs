use core::fmt;
use std::rc::Rc;
use crate::core::Symbols;
use crate::core::value::number::Number;
use crate::core::value::references::class::Property;
use crate::core::value::references::function::UserDefinedFnParam;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct RootNode {
    pub sub_node: ASTNode,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum ASTNode{
    Comment,
    Number(Number),
    String(String),
    Symbol(Symbols),

    Expr(ExprNode),
    LazyExpr(LazyExprNode),
    ArrayLiteral(ArrayLiteralNode),
    MapLiteral(MapLiteralNode),
    Variable(VariableNode),
    Assignment(AssignmentNode),
    ElementReading(ElementReadingNode),
    ObjectReading(ObjectReadingNode),
    Invocation(InvocationNode),
    FunctionDefinition(FunctionDefinitionNode),
    Statement(StatementNode),
    ClassDefinition(ClassDefinitionNode),
    Instantiation(InstantiationNode),
    ImportStatement(ImportNode)
}


impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match self {
            Self::Comment => "Comment",
            Self::Number(_) => "NumberLiteral",
            Self::String(_) => "StringLiteral",
            Self::Symbol(_) => "SymbolLiteral",
            Self::Variable(_) => "Variable",
            Self::Assignment(_) => "Assignment",
            Self::ArrayLiteral(_) => "ArrayLiteral",
            Self::ElementReading(_) => "ElementReading",
            Self::MapLiteral(_) => "MapLiteral",
            Self::Expr(_) => "Expression",
            Self::LazyExpr(_) => "LazyExpression",
            Self::Invocation(_) => "Invocation",
            Self::Statement(_) => "Statement",
            Self::ImportStatement(_) => "ImportStatement",
            Self::FunctionDefinition(_) => "FunctionDefinition",
            Self::ClassDefinition(_) => "ClassDefinition",
            Self::Instantiation(_) => "Instantiation",
            Self::ObjectReading(_) => "ObjectReading",
        };
        write!(f, "(ASTNode: {})", content)
    }
}
pub type ASTVec = Vec<ASTNode>;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct VariableNode {
    pub name: String,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct AssignmentNode {
    pub left_hand_node: ASTNode,
    pub right_hand_node: ExprNode,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ArrayLiteralNode {
    pub elements: Vec<ExprNode>,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct MapLiteralNode {
    pub keys: Vec<String>,
    pub values: Vec<ExprNode>,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ElementReadingNode {
    pub target_node: ASTNode,
    pub index_node: ExprNode,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ExprNode{
    pub elements: ASTVec,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct LazyExprNode {
    pub sub_sequence: ASTNode,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct InvocationNode {
    pub caller: ASTNode,
    pub params: Vec<ExprNode>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ObjectReadingNode {
    pub obj_node: ASTNode,
    pub property: String,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct FunctionDefinitionNode {
    pub params: Vec<UserDefinedFnParam>,
    pub name: Option<String>,
    pub body: ASTVec,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ClassDefinitionNode {
    pub properties: Vec<Property>,
    pub method_nodes: Vec<Rc<FunctionDefinitionNode>>,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct InstantiationNode {
    pub class: String,
    pub params: ArrayLiteralNode,
}

// --- --- --- ---|
// Statement Node |
// --- --- --- ---|
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum StatementNode {
    Output(ExprNode),
    ForLoop(ForStatement),
    Condition(IfStatement),
    Import(ImportNode),
    GlobalAssignment(AssignmentNode),

    Continue,
    Break(ExprNode),
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ForStatement {
    pub loop_count: ExprNode,
    pub body: ASTVec,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct IfStatement {
    pub condition: ExprNode,
    pub body: ASTVec,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum ModuleType {
    BuildIn,
    UserDefined,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ImportNode {
    pub type__: ModuleType,
    pub target: String,
}



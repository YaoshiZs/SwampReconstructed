use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use crate::compilers::analyzer::ast::node::{ASTVec, ExprNode};
use crate::core::err::{range_error, type_error};
use crate::core::std::StdBuildInModule;
use crate::core::value::{into_rc_refcell, Addr, GetAddr};
use crate::core::value::references::Param;
use crate::core::value::value::{Value, ValueType};

#[derive(PartialEq, Clone)]
pub struct BuildInFnParam(pub ValueType, pub &'static str);

impl Param for BuildInFnParam {
    fn _type(&self) -> ValueType {
        self.0
    }
    fn id(&self) -> &str {
        self.1
    }
}

#[derive(PartialEq, Clone)]
pub struct BuildInFunction {
    pub params: Vec<BuildInFnParam>,
    pub id: StdBuildInModule,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct UserDefinedFnParam {
    pub type__: ValueType,
    pub id: String,
}
impl Param for UserDefinedFnParam {
    fn _type(&self) -> ValueType {
        self.type__
    }
    fn id(&self) -> &str {
        &self.id
    }
}
#[derive(PartialEq)]
pub struct UserDefinedFunction {
    pub params: Vec<UserDefinedFnParam>,
    pub body: ASTVec,
}

// --- --- --- --- --- ---

#[derive(PartialEq, Clone)]
pub enum Function {
    BuildIn(Rc<RefCell<BuildInFunction>>),
    UserDefined(Rc<RefCell<UserDefinedFunction>>),
}

impl Function {
    pub fn param_check(
        formal_params: &Vec<impl Param>,
        actual_params: &Vec<ExprNode>,
        expr_resolver: fn(&ExprNode, &mut Scope) -> Result<Value, ()>,
    ) -> Result<(), ()> {
        if actual_params.len() < formal_params.len() {
            // if param missing
            return Err(range_error(
                "function invocation",
                formal_params.len(),
                actual_params.len(),
            )?);
        }

        let mut index = 0;
        while index < formal_params.len() {
            let formal_param = &formal_params[index];

            // compute actual_param_value
            let actual_param_node = (&actual_params[index]).clone();
            let actual_param_value = expr_resolver(actual_param_node.borrow(), whole_scope)?;

            // param type check
            if actual_param_value.check_type(formal_param.type__()) {
                local_scope
                    .variables
                    .insert(formal_param.identi().to_string(), actual_param_value);
            } else {
                type_error(
                    Some(&formal_param.identi()),
                    vec![formal_param.type__()],
                    actual_param_value.get_type(),
                )?
            }

            index += 1;
        }
        Ok(())
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuildIn(_) => write!(f, "<Build-in-Function>"),
            Self::UserDefined(_) => write!(f, "<User-Defined-Function>"),
        }
    }
}
impl GetAddr for Function {
    fn get_addr(&self) -> Addr {
        match self {
            Self::BuildIn(func) => func.as_ptr() as Addr,
            Self::UserDefined(func) => func.as_ptr() as Addr,
        }
    }
}

impl From<UserDefinedFunction> for Function {
    fn from(value: UserDefinedFunction) -> Self {
        Self::UserDefined(into_rc_refcell(value))
    }
}
impl From<BuildInFunction> for Function {
    fn from(value: BuildInFunction) -> Self {
        Self::BuildIn(into_rc_refcell(value))
    }
}
use crate::core::std::std_modules::std_core::StdCore;

mod std_modules;


#[derive(Clone, PartialEq)]
pub enum StdBuildInModule{
    Core(StdCore),
}
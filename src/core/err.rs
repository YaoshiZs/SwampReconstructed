use core::fmt;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use crossterm::style::{StyledContent, Stylize};
use crate::utils::terminal::Terminal;

type ErrorResult = Result<(), ()>;
fn error_name_output(name: &str) -> StyledContent<&str> {
    name.white().on_red().bold()
}

const MATH_ERROR_NAME: &'static str = " MathError ";
pub fn math_error(msg: &str) -> ErrorResult {
    print_line(format!("{}: {}.", error_name_output(MATH_ERROR_NAME), msg));
    Err(())
}

const RANGE_ERROR_NAME: &'static str = " RangeError ";
pub fn range_error<T: Display>(param: &str, expected: T, found: usize) -> ErrorResult {
    print!("{} for \"{}\"", error_name_output(RANGE_ERROR_NAME), param);
    print_line(format!(": expected {}, found {}.", expected, found));
    Err(())
}

const SYNTAX_ERROR_NAME: &'static str = " SyntaxError ";
pub fn syntax_error(msg: &str) -> ErrorResult {
    print_line(format!(
        "{}: {}.\r",
        error_name_output(SYNTAX_ERROR_NAME),
        msg
    ));
    Err(())
}

const ASSIGNMENT_ERROR_NAME: &'static str = " SyntaxError ";
pub fn assignment_error(msg: &str) -> ErrorResult {
    print_line(format!(
        "{}: {}.",
        error_name_output(ASSIGNMENT_ERROR_NAME),
        msg
    ));
    Err(())
}

const REFERENCE_ERROR_NAME: &'static str = " ReferenceError ";
pub enum ReferenceType {
    Variable,
    Property,
}
pub fn reference_error(type__: ReferenceType, target_name: &str) -> ErrorResult {
    print_line(format!(
        "{}: {} `{}` is not defined.",
        error_name_output(REFERENCE_ERROR_NAME),
        match type__ {
            ReferenceType::Variable => "variable",
            ReferenceType::Property => "property",
        },
        target_name,
    ));
    Err(())
}

const IMPORT_ERROR_NAME: &'static str = " ImportError ";
pub fn import_error(msg: &str) -> ErrorResult {
    print_line(format!(
        "{}: {}.",
        error_name_output(IMPORT_ERROR_NAME),
        msg
    ));
    Err(())
}

// --- --- --- --- --- ---

pub enum InternalComponent {
    Std,
    InternalFn,

    Tokenizer,
    Analyzer,
    Computer,
}
impl Display for InternalComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalComponent::Std => write!(f, "Standard-Library"),
            InternalComponent::InternalFn => write!(f, "Internal-Function"),

            InternalComponent::Tokenizer => write!(f, "Tokenizer"),
            InternalComponent::Analyzer => write!(f, "Analyzer"),
            InternalComponent::Computer => write!(f, "Computer"),
        }
    }
}

const INTERNAL_ERROR_NAME: &'static str = " InternalError ";
pub fn internal_error(from: InternalComponent, msg: &str) -> ErrorResult {
    print_line(format!(
        "{} from {}: {}.",
        error_name_output(INTERNAL_ERROR_NAME),
        from,
        msg
    ));
    Err(())
}


// this function is used to replace Rust macro `println!`
// since the println! macro can not normally
// make new line in raw_mode.
pub fn print_line<T: Display>(content: T) {
    print!("{}\r\n", content);
    Terminal::flush().expect("IO Error");
}

// output something into file
// this function is used to debug.
pub fn log(content: &str) -> io::Result<()> {
    File::create("log.txt")?;
    let mut file = OpenOptions::new().write(true).open("log.txt")?;
    file.write(content.as_bytes())?;
    file.flush()?;
    Ok(())
}




pub mod arg_parser;

use crate::btmd;

use btmd::values::ValueTypes;

pub use arg_parser::ArgParser;

#[derive(Clone, Default)]
pub struct ArgPreset {
    pub(crate) vec: Vec<ValueTypes>,
}

impl ArgPreset {
    pub fn new(vec: Vec<ValueTypes>) -> Self {
        Self { vec }
    }
}

#[macro_export]
macro_rules! args_parser {
    ($($name:expr),+) => {
        $crate::btmd::args::ArgParser::new($crate::btmd::args::ArgPreset::new(vec![$($name),+]))
    };
}

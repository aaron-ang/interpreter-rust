pub const THIS_KEYWORD: &str = "this";
pub const SUPER_KEYWORD: &str = "super";
pub const INIT_METHOD: &str = "init";

pub mod errors {
    pub const OPERANDS_MUST_BE_NUMBERS: &str = "Operands must be numbers.";
    pub const OPERANDS_MUST_BE_NUMBERS_OR_STRINGS: &str = "Operands must be numbers or strings.";
    pub const ONLY_INSTANCES_HAVE_FIELDS: &str = "Only instances have fields.";
    pub const RETURN_FROM_INITIALIZER: &str = "Can't return a value from an initializer.";
    pub const RETURN_FROM_TOP_LEVEL: &str = "Can't return from top-level code.";
    pub const THIS_OUTSIDE_CLASS: &str = "Can't use 'this' outside of a class.";
    pub const SUPER_OUTSIDE_CLASS: &str = "Can't use 'super' outside of a class.";
    pub const SUPER_WITHOUT_SUPERCLASS: &str = "Can't use 'super' in a class with no superclass.";
    pub const CLASS_INHERIT_SELF: &str = "A class can't inherit from itself.";
    pub const VARIABLE_IN_OWN_INITIALIZER: &str =
        "Can't read local variable in its own initializer.";
    pub const DUPLICATE_VARIABLE: &str = "Already a variable with this name in this scope.";
}

pub mod exit_codes {
    pub const COMMAND_LINE_USAGE: i32 = 64;
    pub const SYNTAX_ERROR: i32 = 65;
    pub const CANNOT_OPEN_INPUT: i32 = 66;
    pub const RUNTIME_ERROR: i32 = 70;
}

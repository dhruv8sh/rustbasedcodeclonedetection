use crate::lib::grammar::user_defined_members::{ClassOrRecord, Enum, Interface};
use expression::*;
use std::fmt::{Display, Formatter};

pub(crate) enum Statement {
    Assignment(Variable, Expression),
    Declaration(Variable),
    ClassDeclaration(),
    Import(String),
    Package(String),
    Comment(String),
}
// impl Display for Statement {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         writeln!(
//             f,
//             "{}",
//             match self {
//                 Statement::Assignment(a, b) => format!("Assignment: {}={}", a.name, b),
//                 Statement::Declaration(a) => format!("Assignment: {} of {}", a.name, a.kind),
//                 Statement::Import(item) => format!("Imported: {item}"),
//                 Statement::Package(pack) => format!("Inside Package: {pack}"),
//                 Statement::Comment(content) => format!("Comment : {content}"),
//             }
//         )
//     }
// }

pub(crate) struct JavaCodeFile {
    pub(crate) package: Option<String>,
    pub(crate) imports: Option<Vec<String>>,
    pub(crate) classes: Vec<ClassOrRecord>,
    pub(crate) enums: Vec<Enum>,
    pub(crate) interfaces: Vec<Interface>,
}

pub(crate) mod statements {
    use super::user_defined_members::*;
    use crate::lib::grammar::expression::{Expression, Variable};
    use crate::lib::grammar::Statement;

    pub(crate) enum JumpStatements {
        Break,
        Return,
        Continue,
        Throw,
    }
    pub(crate) enum LoopStatements {
        For(
            DeclarationStatements,
            ConditionalStatements,
            UpdateStatements,
        ),
        While,
        DoWhile,
    }
    pub(crate) enum ConditionalStatements {
        If(Expression),
        Else,
        Switch(Expression, Vec<Statement>),
    }
    pub(crate) enum UpdateStatements {
        Assignment(Variable, Expression),
        Increment(Variable),
        Decrement(Variable),
        MethodCall(&'static Method),
    }
    pub(crate) enum DeclarationStatements {
        Declaration(Variable),
        Definition(Variable, Expression),
        ClassDeclare,
        EnumDeclare,
        InterfaceDeclare,
    }
}

pub(crate) mod expression {
    use crate::lib::grammar::expression::Expression::Value;
    use crate::lib::grammar::expression::Modifiers::*;
    use std::fmt::{Display, Formatter};

    pub(crate) enum Operation {
        Add(Expression, Expression),
        Subtract(Expression, Expression),
        Multiply(Expression, Expression),
        Divide(Expression, Expression),
        Mod(Expression, Expression),
        Increment(Expression),
        Decrement(Expression),
        UnaryAdd(Expression),
        UnarySubtract(Expression),
        Equals(Expression, Expression),
        NotEquals(Expression, Expression),
        GreaterThanOrEqual(Expression, Expression),
        GreaterThan(Expression, Expression),
        LesserThanOrEqual(Expression, Expression),
        LesserThan(Expression, Expression),
        LShift(Expression, Expression),
        RShift(Expression, Expression),
        UnsignedShiftRight(Expression, Expression),
        Complement(Expression),
        Not(Expression),
        And(Expression, Expression),
        Or(Expression, Expression),
        XOR(Expression, Expression),
        BitwiseOr(Expression, Expression),
        BitwiseAnd(Expression, Expression),
        Ternary(Expression, Expression, Expression),
        InstanceOf(Expression, DataKind),
        MemberAccess(Variable, Variable),
    }
    impl ToString for Operation {
        fn to_string(&self) -> String {
            match self {
                Operation::Add(a, b) => format!("{a}+{b}"),
                Operation::Subtract(a, b) => format!("{a}-{b}"),
                Operation::Multiply(a, b) => format!("{a}*{b}"),
                Operation::Divide(a, b) => format!("{a}/{b}"),
                Operation::Mod(a, b) => format!("{a}%{b}"),
                Operation::Increment(a) => format!("{a}++"),
                Operation::Decrement(a) => format!("{a}--"),
                Operation::UnaryAdd(a) => format!("{a}+"),
                Operation::UnarySubtract(a) => format!("{a}-"),
                Operation::Equals(a, b) => format!("{a}=={b}"),
                Operation::NotEquals(a, b) => format!("{a}!={b}"),
                Operation::GreaterThanOrEqual(a, b) => format!("{a}>={b}"),
                Operation::GreaterThan(a, b) => format!("{a}>{b}"),
                Operation::LesserThanOrEqual(a, b) => format!("{a}<={b}"),
                Operation::LesserThan(a, b) => format!("{a}<{b}"),
                Operation::LShift(a, b) => format!("{a}<<{b}"),
                Operation::RShift(a, b) => format!("{a}>>{b}"),
                Operation::UnsignedShiftRight(a, b) => format!("{a}>>>{b}"),
                Operation::Complement(a) => format!("!{a}"),
                Operation::Not(a) => format!("~{a}"),
                Operation::And(a, b) => format!("{a}&&{b}"),
                Operation::Or(a, b) => format!("{a}||{b}"),
                Operation::XOR(a, b) => format!("{a}^{b}"),
                Operation::BitwiseOr(a, b) => format!("{a}|{b}"),
                Operation::BitwiseAnd(a, b) => format!("{a}&{b}"),
                Operation::Ternary(cond, a, b) => format!("{cond}?{a}:{b}"),
                Operation::InstanceOf(a, b) => format!("{a} instanceof {b}"),
                Operation::MemberAccess(a, b) => format!("{a}.{b}"),
            }
            .to_string()
        }
    }
    pub(crate) struct Literal {
        kind: DataKind,
        value: String,
    }

    impl Display for Literal {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})", self.value)
        }
    }
    pub(crate) enum DataKind {
        Number,
        CharSeq,
        Boolean,
        Void,
        Array(Box<DataKind>),
        ClassOrRecord,
        Enum,
        Interface,
    }
    impl Display for DataKind {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", stringify!(self))
        }
    }
    pub(crate) enum Modifiers {
        Public,
        Private,
        Protected,
        Default,
        Final,
        Abstract,
        Static,
        Synchronized,
        Native,
        Strictfp,
        Volatile,
        Transient,
    }
    impl TryFrom<&str> for Modifiers {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Ok(match value {
                "public" => Public,
                "private" => Private,
                "protected" => Protected,
                "final" => Final,
                "abstract" => Abstract,
                "static" => Static,
                "synchronized" => Synchronized,
                "native" => Native,
                "strictfp" => Strictfp,
                "volatile" => Volatile,
                "transient" => Transient,
                _ => return Err(()),
            })
        }
    }
    pub(crate) struct Variable {
        pub(crate) id: u32,
        pub(crate) kind: DataKind,
        pub(crate) name: String,
        pub(crate) modifiers: Vec<Modifiers>,
        pub(crate) value: Box<Expression>,
    }

    // impl TryFrom<&Vec<&str>> for Variable {
    //     type Error = ();
    // }

    impl Display for Variable {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} of {}", self.name, self.kind)
        }
    }
    pub(crate) enum Expression {
        Value(Literal),
        Variable(Variable),
        Operation(Box<Operation>),
    }
    impl Display for Expression {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Ok(match self {
                Value(a) => write!(f, "{a}")?,
                Expression::Variable(a) => write!(f, "{a}")?,
                Expression::Operation(a) => write!(f, "{}", (*a).to_string())?,
            })
        }
    }
}

pub mod user_defined_members {
    use super::expression::*;
    pub(crate) struct ClassOrRecord {
        // pub(crate) id: Id,
        pub(crate) data_members: Vec<Variable>,
        pub(crate) methods: Vec<Method>,
        pub(crate) extends: Option<Vec<ClassOrRecord>>,
        pub(crate) implements: Option<Interface>,
        pub(crate) inner_classes: Option<Vec<ClassOrRecord>>,
        pub(crate) inner_enum: Option<Vec<Enum>>,
        pub(crate) inner_interface: Option<Vec<Interface>>,
    }
    pub(crate) struct Enum {
        pub(crate) var: Variable,
        pub(crate) enumerations: Vec<String>,
        pub(crate) data_members: Vec<Variable>,
        pub(crate) methods: Vec<Method>,
        pub(crate) extends: Option<Vec<Enum>>,
        pub(crate) implements: Option<Interface>,
    }
    pub(crate) struct Interface {
        pub(crate) var: Variable,
        pub(crate) data_members: Vec<Variable>,
        pub(crate) methods: Vec<Method>,
        pub(crate) extends: Option<Vec<Interface>>,
    }
    pub(crate) struct Method {
        pub(crate) var: Variable,
        pub(crate) throws: Vec<String>,
        pub(crate) input: Vec<Variable>,
    }
}

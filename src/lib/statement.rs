enum Statement {
    Declaration,
    MethodCall(u32),
    Assignment(Variable, Expression),
    If(Condition),
    ElseIf(Condition),
    Else,
    Switch(Expression),
    SwitchCase(Literal),
    While(Expression),
    For(Expression),
    DoWhile(Expression),
    Break(Tag),
    Continue(Tag),
    Return(Expression),
}
struct Tag {
    id: Id,
    name: String,
}
enum IdKind {
    Class,
    Tag,
    Function,
}
struct Id {
    id: String,
}
impl Id {
    fn new(kind: IdKind) -> Self {
        Id {
            id: match kind {
                IdKind::Class => "class",
                IdKind::Tag => "tag",
                IdKind::Function => "function",
            }
            .to_string(),
        }
    }
}

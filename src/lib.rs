use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // Keywords
    #[token("function")]
    FunctionKw,

    #[token("local")]
    LocalKw,

    #[token("true")]
    TrueKw,

    #[token("false")]
    FalseKw,

    #[token("if")]
    IfKw,

    #[token("then")]
    ThenKw,

    #[token("else")]
    ElseKw,

    #[token("elseif")]
    ElseIfKw,

    #[token("while")]
    WhileKw,

    #[token("for")]
    ForKw,

    #[token("in")]
    InKw,

    #[token("break")]
    BreakKw,

    #[token("do")]
    DoKw,

    #[token("goto")]
    GotoKw,

    #[token("and")]
    AndKw,

    #[token("or")]
    OrKw,

    #[token("not")]
    NotKw,

    #[token("return")]
    ReturnKw,

    #[token("end")]
    EndKw,

    #[token("repeat")]
    RepeatKw,

    #[token("until")]
    UntilKw,

    #[token("nil")]
    NilKw,

    // Puncuation
    #[token("(")]
    LParen,

    #[token(")")]
    Rparen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("..")]
    DoubleDot,

    #[token("...")]
    TripleDot,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[token("::")]
    DoubleColon,

    // Operators
    #[token("=")]
    Eq,

    #[token("==")]
    EqEq,

    #[token("~=")]
    NotEq,

    #[token("#")]
    Hash,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token(">")]
    Gt,

    #[token("<")]
    Lt,

    #[token(">=")]
    GtEq,

    #[token("<=")]
    LtEq,

    #[token("!")]
    Bang,

    #[regex(r#""(?:\\"|\\'|[^"])*""#)]
    #[regex(r#"'(?:\\"|\\'|[^'])*'"#)]
    Str,

    #[regex("[a-zA-Z_][a-zA-Z_0-9]*")]
    Ident,

    #[regex(r"\s+")]
    Whitespace,

    #[regex("0x[0-9a-f]")]
    #[regex(r"\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?")]
    Number,

    #[regex("--.*")]
    Comment,

    #[regex("---.*")]
    DocComment,

    #[error]
    Error,
}

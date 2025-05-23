use crate::syntax::ast::{Id, Span, Spanned, Type};
use crate::syntax::lexer::Token;

#[derive(Debug, Clone)]
pub enum AgudaError {
    Compile(CompileError),
    Runtime(RuntimeError),
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum CompileError {
    Lexical(LexicalError),
    Syntax(SyntaxError),
    Semantic(SemanticError),
}

#[derive(Debug, Clone)]
pub struct LexicalError {
    pub kind: LexicalErrorKind,
    pub span: Span,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalErrorKind {
    InvalidInteger,
    IntegerOverflow,
    FloatingPointNumber,
    UnterminatedString,
    #[default]
    UnrecognizedToken,
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxErrorKind {
    UnexpectedToken(Vec<String>, Token),
    UnexpectedEof(Vec<String>),
    InvalidToken,
    ExtraToken,
}

#[derive(Debug, Clone)]
pub enum SemanticError {
    Declaration(DeclarationError),
    Type(TypeError),
}

#[derive(Debug, Clone)]
pub struct DeclarationError {
    pub kind: DeclarationErrorKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum DeclarationErrorKind {
    UndeclaredIdentifier(Id, Option<Id>),
    DuplicateDeclaration(Id),
    ReservedIdentifier(Id),
    FunctionSignatureMismatch {
        params_found: usize,
        types_found: usize,
    },
    DuplicateMain,
    MissingMain,
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub kind: TypeErrorKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TypeErrorKind {
    TypeMismatch {
        found: Type,
        expected: Type,
    },
    IncompatibleTypes(Type, Type),
    ArgumentCountMismatch {
        found: usize,
        expected: usize,
    },
    NotCallable {
        found: Type
    },
    NotIndexable {
        found: Type
    },
    MainSignatureMismatch,
}

impl LexicalError {
    pub fn invalid_integer(span: Span) -> Self {
        Self {
            kind: LexicalErrorKind::InvalidInteger,
            span,
        }
    }

    pub fn integer_overflow(span: Span) -> Self {
        Self {
            kind: LexicalErrorKind::IntegerOverflow,
            span,
        }
    }

    pub fn floating_point_number(span: Span) -> Self {
        Self {
            kind: LexicalErrorKind::FloatingPointNumber,
            span,
        }
    }

    pub fn unterminated_string(span: Span) -> Self {
        Self {
            kind: LexicalErrorKind::UnterminatedString,
            span,
        }
    }

    pub fn unrecognized_token(span: Span) -> Self {
        Self {
            kind: LexicalErrorKind::UnrecognizedToken,
            span,
        }
    }
}

impl From<LexicalError> for CompileError {
    fn from(e: LexicalError) -> Self {
        CompileError::Lexical(e)
    }
}


impl SyntaxError {
    pub fn unexpected_token(span: Span, expected: Vec<String>, found: Token) -> Self {
        Self {
            kind: SyntaxErrorKind::UnexpectedToken(expected, found),
            span,
        }
    }

    pub fn unexpected_eof(span: Span, expected: Vec<String>) -> Self {
        Self {
            kind: SyntaxErrorKind::UnexpectedEof(expected),
            span,
        }
    }

    pub fn invalid_token(span: Span) -> Self {
        Self {
            kind: SyntaxErrorKind::InvalidToken,
            span,
        }
    }

    pub fn extra_token(span: Span) -> Self {
        Self {
            kind: SyntaxErrorKind::ExtraToken,
            span,
        }
    }
}

impl From<SyntaxError> for CompileError {
    fn from(e: SyntaxError) -> Self {
        CompileError::Syntax(e)
    }
}

impl SemanticError {
    pub fn from_both(
        decl_errors: Vec<DeclarationError>,
        type_errors: Vec<TypeError>
    ) -> Vec<SemanticError> {
        decl_errors
            .into_iter()
            .map(SemanticError::Declaration)
            .chain(type_errors.into_iter().map(SemanticError::Type))
            .collect()
    }
}

impl From<SemanticError> for CompileError {
    fn from(e: SemanticError) -> Self {
        CompileError::Semantic(e)
    }
}

impl DeclarationError {
    pub fn undeclared_identifier(spanned: Spanned<Id>, similar: Option<Id>) -> Self {
        Self {
            kind: DeclarationErrorKind::UndeclaredIdentifier(spanned.value, similar),
            span: spanned.span,
        }
    }

    pub fn duplicate_declaration(spanned: Spanned<Id>) -> Self {
        Self {
            kind: DeclarationErrorKind::DuplicateDeclaration(spanned.value),
            span: spanned.span,
        }
    }

    pub fn reserved_identifier(spanned: Spanned<Id>) -> Self {
        Self {
            kind: DeclarationErrorKind::ReservedIdentifier(spanned.value),
            span: spanned.span,
        }
    }

    pub fn function_signature_mismatch(span: Span, params_found: usize, types_found: usize) -> Self {
        Self {
            kind: DeclarationErrorKind::FunctionSignatureMismatch {
                params_found,
                types_found,
            },
            span,
        }
    }

    pub fn duplicate_main(span: Span) -> Self {
        Self {
            kind: DeclarationErrorKind::DuplicateMain,
            span,
        }
    }

    pub fn missing_main() -> Self {
        Self {
            kind: DeclarationErrorKind::MissingMain,
            span: Span::default(),
        }
    }
}

impl From<DeclarationError> for CompileError {
    fn from(e: DeclarationError) -> Self {
        CompileError::Semantic(SemanticError::Declaration(e))
    }
}

impl TypeError {
    pub fn type_mismatch(span: Span, found: Type, expected: Type) -> Self {
        Self {
            kind: TypeErrorKind::TypeMismatch { found, expected },
            span,
        }
    }

    pub fn expected_equal_types(span: Span, lhs: Type, rhs: Type) -> Self {
        Self {
            kind: TypeErrorKind::IncompatibleTypes(lhs, rhs),
            span,
        }
    }

    pub fn arg_count_mismatch(span: Span, found: usize, expected: usize) -> Self {
        Self {
            kind: TypeErrorKind::ArgumentCountMismatch { found, expected },
            span,
        }
    }

    pub fn not_callable(span: Span, found: Type) -> Self {
        Self {
            kind: TypeErrorKind::NotCallable { found },
            span,
        }
    }

    pub fn not_indexable(span: Span, found: Type) -> Self {
        Self {
            kind: TypeErrorKind::NotIndexable { found },
            span,
        }
    }

    pub fn main_signature_mismatch(span: Span) -> Self {
        Self {
            kind: TypeErrorKind::MainSignatureMismatch,
            span,
        }
    }
}

impl From<TypeError> for CompileError {
    fn from(e: TypeError) -> Self {
        CompileError::Semantic(SemanticError::Type(e))
    }
}

impl From<CompileError> for AgudaError {
    fn from(e: CompileError) -> Self {
        AgudaError::Compile(e)
    }
}

impl From<RuntimeError> for AgudaError {
    fn from(e: RuntimeError) -> Self {
        AgudaError::Runtime(e)
    }
}
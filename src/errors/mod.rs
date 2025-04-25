use crate::syntax::ast::{Id, Span, Spanned, Type};

pub mod formatting;

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

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxErrorKind {
    UnexpectedToken(Vec<String>),
    UnexpectedEof(Vec<String>),
    InvalidToken,
    ExtraToken,
}

impl SyntaxError {
    pub fn unexpected_token(span: Span, expected: Vec<String>) -> Self {
        Self {
            kind: SyntaxErrorKind::UnexpectedToken(expected),
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

pub enum SemanticError {
    Declaration(DeclarationError),
    Type(TypeError),
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

#[derive(Debug, Clone)]
pub struct DeclarationError {
    pub kind: DeclarationErrorKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum DeclarationErrorKind {
    UndeclaredSymbol(Id),
    ReservedIdentifier(Id),
    WrongFunctionSignature {
        params_found: usize,
        types_found: usize,
    },
}

impl DeclarationError {
    pub fn undeclared_identifier(spanned: Spanned<Id>) -> Self {
        Self {
            kind: DeclarationErrorKind::UndeclaredSymbol(spanned.value),
            span: spanned.span,
        }
    }

    pub fn reserved_identifier(spanned: Spanned<Id>) -> Self {
        Self {
            kind: DeclarationErrorKind::ReservedIdentifier(spanned.value),
            span: spanned.span,
        }
    }

    pub fn wrong_function_signature(span: Span, params_found: usize, types_found: usize) -> Self {
        Self {
            kind: DeclarationErrorKind::WrongFunctionSignature {
                params_found,
                types_found,
            },
            span,
        }
    }
}

impl From<DeclarationError> for CompileError {
    fn from(e: DeclarationError) -> Self {
        CompileError::Semantic(SemanticError::Declaration(e))
    }
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
    WrongNumberOfArguments {
        found: usize,
        expected: usize,
    },
    NotCallable {
        found: Type
    },
    NotIndexable {
        found: Type
    },
}

impl TypeError {
    pub fn type_mismatch(span: Span, found: Type, expected: Type) -> Self {
        Self {
            kind: TypeErrorKind::TypeMismatch { found, expected },
            span,
        }
    }

    pub fn wrong_num_of_args(span: Span, found: usize, expected: usize) -> Self {
        Self {
            kind: TypeErrorKind::WrongNumberOfArguments { found, expected },
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
}

impl From<TypeError> for CompileError {
    fn from(e: TypeError) -> Self {
        CompileError::Semantic(SemanticError::Type(e))
    }
}

#[derive(Debug, Clone)]
pub enum Warning {
    UnusedSymbol(Spanned<Id>),
    DuplicateDeclaration(Spanned<Id>),
}
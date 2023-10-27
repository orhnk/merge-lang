#[allow(unused)]
pub enum InLang {
    C,
    CSharp,
    Cpp,
    Cobol,
    Carbon,
    D,
    Go,
    Haskell,
    Merge,
    OCaml,
    Python,
    R,
    Rust,
    Racket,
    V,
}

impl InLang {
    // TODO: this is ambigious by the case size (upper - lower)
    fn from(lang: &str) -> Option<Self> {
        return match lang {
            // TODO: Compile time manipulate this to lowercase
            "c" => Some(Self::C),
            "csharp" => Some(Self::CSharp),
            "cpp" => Some(Self::Cpp),
            "cobol" => Some(Self::Cobol),
            "carbon" => Some(Self::Carbon),
            "d" => Some(Self::D),
            "go" => Some(Self::Go),
            "haskell" => Some(Self::Haskell),
            "merge" => Some(Self::Merge),
            "ocaml" => Some(Self::OCaml),
            "python" => Some(Self::Python),
            "r" => Some(Self::R),
            "rust" => Some(Self::Rust),
            "racket" => Some(Self::Racket),
            "v" => Some(Self::V),
            _ => None,
        };
    }
}

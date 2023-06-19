use super::*;

pub fn type_to_string(type_: &Type) -> String {
    match type_ {
        Type::Unit => "()".to_string(),
        Type::Path(path) => path_to_string(path),
        Type::Complex(complex) => complex_to_string(complex),
    }
}

pub fn path_to_string(path: &[Ident]) -> String {
    path.iter()
        .map(|part| -> &str { &part.name })
        .collect::<Vec<&str>>()
        .join("::")
}

pub fn complex_to_string(complex: &ComplexType) -> String {
    let callee = path_to_string(&complex.callee);
    let comma_separated_args = complex
        .args
        .iter()
        .map(type_to_string)
        .collect::<Vec<String>>()
        .join(", ");
    format!("{callee}<{comma_separated_args}>")
}

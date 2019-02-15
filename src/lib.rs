extern crate lexer;
extern crate parser;
extern crate types;
extern crate ast;
extern crate errors;

use lexer::Lexer;
use parser::DeclarationParser;
use types::{StorageClass, QualifiedType};
use errors::CompilationError;
use types::type_resolution::ResolveDeclarator;

struct DeclaredType {
    identifier: Option<String>,
    storage_class: StorageClass,
    qualified_type : QualifiedType
}

impl DeclaredType {
    pub fn new(identifier: Option<String>, storage_class: StorageClass, qualified_type: QualifiedType) -> DeclaredType {
        DeclaredType {
            identifier,
            storage_class,
            qualified_type
        }
    }
}

fn parse_declarations<'a>(source: &'a str) -> Result<Vec<DeclaredType>, CompilationError<'a>> {
    let lexer = Lexer::new("declaration.c".into(), source);
    let declaration = DeclarationParser::new().parse(lexer)?;

    let storage_class = types::StorageClass::try_from_specifier_list(&declaration.declaration_specifiers)?;
    let qualified_type = types::QualifiedType::from_declaration_specifier_list(&declaration.declaration_specifiers)?;

    let mut results = Vec::new();

    for declarator in declaration.init_declarator_list.into_iter().map(|v| v.declarator) {
        let (type_, identifier) = declarator.resolve(qualified_type.clone());
        results.push(DeclaredType::new(identifier, storage_class, type_));
    }

    Ok(results)
}

pub fn describe_declarators<'a>(source: &'a str) -> String {
    match parse_declarations(source) {
        Ok(list) => {
            let mut declarations = Vec::new();
            for declaration in list {
                if let Some(identifier) = declaration.identifier {
                    if declaration.storage_class == StorageClass::Auto {
                        declarations.push(format!("declare {} as {}", identifier, declaration.qualified_type));
                    }
                    else {
                        declarations.push(format!("declare {} as {} {}", identifier, declaration.storage_class, declaration.qualified_type));
                    }
                }
            }
            declarations.join("\n")
        },
        Err(e) => format!("{}", e)
    }
}

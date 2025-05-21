enum Primitive {
    Int,
    Float,
    Bit,
    Byte,
    Bool,
}

enum TypeKind {
    Primitive,
    Struct,
    Array,
    Function,
    Alias,
    TemplateInstance,
    TemplateParam,
    Tuple,
    Enum,
    Union,
}

fn return_result(input: bool) -> Result<bool, String> {
    if input {
        return Ok(true);
    }

    return Err("Hello".to_string());
}

fn main() {
    let x = return_result(false);
    match x {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Error"),
    }

    let x = true;
    match x {
        true  => println!("True"),
        false => println!("False"),
    }
}

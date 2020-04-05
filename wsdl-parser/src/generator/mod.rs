use crate::parser::definitions::Definitions;
use crate::generator::function::{Function, Param};
use crate::parser::port_type::OperationType;

pub mod function;


pub fn generate<'a>(definitions: &'a Definitions<'_>) -> Vec<Function<'a>> {
    use OperationType::*;
    let mut res = vec![];
    for (name, port_type) in definitions.port_types() {
        for op in port_type.operations(){
            let mut input_message = None;
            let mut output_message = None;
            match op.operation_type() {
                RequestResponse{input, output,..} => {
                    input_message = definitions.get_message_by_param(input);
                    output_message = definitions.get_message_by_param(output);
                }
                OneWay{input} => {input_message = definitions.get_message_by_param(input)}
                _ => { unreachable!("No present in ONVIF"); }
            };


            let func = Function {
                generic_params: vec![Param{ name: "T", typename: "transport::Transport"}],
                arguments: vec![
                    Param{ name: "transport", typename: "T"},
                    Param{name: "request", typename: input_message.unwrap().parts()[0].element().unwrap()}
                ],
                return_type: output_message.and_then(|m| m.parts()[0].element()).unwrap_or(""),
                documentation: "",
                name: op.name()
            };
            println!("{}", print_function(&func));
            res.push(func);
        }
    }
    res
}

const REQUEST_FUNC_BODY: &'static str = "transport::request(transport, request).await";


pub fn print_function(func: &Function<'_>) -> String {
    format!(r#"
pub async fn {name}<{generics}>(
    {arguments}
) -> Result<{return_type}, transport::Error> {{
    {body}
}}
"#,
        name = func.name,
        generics = func.generic_params.iter().map(|p| format!("{}: {}", p.name, p.typename)).collect::<Vec<String>>().join(", "),
        arguments = func.arguments.iter().map(|p| format!("{}: &{}", p.name, p.typename)).collect::<Vec<String>>().join(",\n"),
        return_type = func.return_type,
        body = REQUEST_FUNC_BODY
    )
}



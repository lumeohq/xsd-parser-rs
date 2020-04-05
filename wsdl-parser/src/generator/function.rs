use crate::parser::definitions::Definitions;
use crate::parser::message::{Message, Part};
use crate::parser::port_type::Operation;
use crate::parser::port_type::OperationType::{OneWay, RequestResponse};

const GENERIC_TRANSPORT: Param<'static> = Param {
    name: "T",
    typename: "transport::Transport",
};
const ARGUMENT_TRANSPORT: Param<'static> = Param {
    name: "transport",
    typename: "T",
};

#[derive(Debug)]
pub struct Function<'a> {
    pub generic_params: Vec<Param<'a>>,
    pub arguments: Vec<Param<'a>>,
    pub return_type: &'a str,
    pub documentation: Option<&'a str>,
    pub name: &'a str,
}

#[derive(Debug, Clone)]
pub struct Param<'a> {
    pub name: &'a str,
    pub typename: &'a str,
}

impl<'a> Param<'a> {
    pub fn new(part: &'a Part<'_>) -> Self {
        Param {
            name: part.name(),
            typename: part
                .element()
                .or_else(|| part.type_())
                .expect("Element or type must be presented in wsdl:part"),
        }
    }
}

impl<'a> Function<'a> {
    pub fn new(operation: &'a Operation<'_>, definitions: &'a Definitions<'_>) -> Self {
        let input_message;
        let mut output_message = None;
        match operation.operation_type() {
            RequestResponse { input, output, .. } => {
                input_message = definitions.get_message_by_param(input);
                output_message = definitions.get_message_by_param(output);
            }
            OneWay { input } => input_message = definitions.get_message_by_param(&input),
            _ => {
                unreachable!("No present in ONVIF");
            }
        };

        Function {
            generic_params: vec![GENERIC_TRANSPORT],
            arguments: get_input_params(input_message),
            return_type: output_message
                .and_then(|m| m.parts()[0].element())
                .unwrap_or(""),
            documentation: operation.documentation(),
            name: operation.name(),
        }
    }
}

fn get_input_params<'a>(input: Option<&'a Message<'_>>) -> Vec<Param<'a>> {
    let mut result = vec![ARGUMENT_TRANSPORT];
    if input.is_none() {
        return result;
    }
    let input = input.unwrap();

    if input.parts().len() == 1 {
        // ONVIF case
        result.push(Param {
            name: "request",
            typename: input.parts()[0].element().unwrap(),
        })
    } else {
        result.append(&mut input.parts().iter().map(Param::new).collect());
    };
    result
}

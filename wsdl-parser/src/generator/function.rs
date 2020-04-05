


#[derive(Debug)]
pub struct Function<'a> {
    pub generic_params: Vec<Param<'a>>,
    pub arguments: Vec<Param<'a>>,
    pub return_type: &'a str,
    pub documentation: &'a str,
    pub name: &'a str,
}

#[derive(Debug)]
pub struct Param<'a>{
    pub name: &'a str,
    pub typename: &'a str
}


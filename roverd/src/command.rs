use crate::error;

#[derive(Debug)]
pub struct ParsedCommand {
    pub program: String,
    pub arguments: Vec<String>,
}

impl TryFrom<&String> for ParsedCommand {
    type Error = error::Error;

    fn try_from(value: &String) -> Result<Self, error::Error> {
        let command = value.clone();
        let separated: Vec<&str> = command.split_whitespace().collect();
        let (program, arguments) = separated
            .split_first()
            .ok_or_else(|| error::Error::RunCommandNotParsed)?;
        let program = program.to_string();
        let arguments: Vec<String> = arguments.iter().map(|a| a.to_string()).collect();

        Ok(Self { program, arguments })
    }
}

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Arguments {
    pub command: String,
    pub subcommand: Option<String>,
}

impl Arguments {
    pub fn parse() -> Result<Self> {
        let mut args = std::env::args();
        args.next();

        let command = match args.next() {
            Some(arg) => arg,
            None => String::from("explorer"),
        };

        let subcommand = match args.next() {
            Some(arg) => Some(arg),
            None => None,
        };

        Ok(Self {
            command,
            subcommand,
        })
    }
}

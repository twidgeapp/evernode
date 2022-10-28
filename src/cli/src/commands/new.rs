use super::{Command, Commands};
use anyhow::Result;
use async_trait::async_trait;
use requestty::Question;

pub struct New {}

impl New {
    pub fn new() -> Self {
        Self {}
    }

    pub fn name_prompt(&self) -> Result<String> {
        let default_name = names::Generator::with_naming(names::Name::Plain)
            .next()
            .unwrap();

        let db_name_propmpt = Question::input("Enter a name for your database")
            .default(default_name)
            .build();

        let db_name = requestty::prompt_one(db_name_propmpt)?;

        Ok(db_name.as_string().unwrap().to_string())
    }
}

#[async_trait]
impl Command for New {
    type Output = String;

    async fn exec(&self, commands: Commands) -> Result<String> {
        let db_name = if let Some(sub_cmd) = commands.shared.argument.subcommand {
            sub_cmd
        } else {
            self.name_prompt()?
        };

        Ok(String::new())
    }
}

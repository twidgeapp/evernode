use super::{Command, Commands};
use anyhow::Result;
use async_trait::async_trait;
use requestty::Question;

pub struct New {}

impl New {
    pub fn new() -> Self {
        Self {}
    }

    pub fn query_input(&self, query: &str, default: Option<String>) -> Result<String> {
        let mut prompt = None;

        if let Some(d) = default {
            prompt = Some(Question::input(query).default(d).build());
        } else {
            prompt = Some(Question::input(query).build());
        }

        let prompt = requestty::prompt_one(prompt.unwrap())?;

        Ok(prompt.as_string().unwrap().to_string())
    }

    pub fn query_multi_select(&self, options: Vec<&str>, query: &str) -> Result<String> {
        let prompt = Question::select(query).choices(options).build();
        let prompt = requestty::prompt_one(prompt)?;
        let answer = prompt.clone();
        let item = answer.as_list_item().unwrap();
        let output = item.text.clone();

        Ok(output)
    }
}

#[async_trait]
impl Command for New {
    type Output = String;

    async fn exec(&self, commands: Commands) -> Result<String> {
        let db_name = if let Some(sub_cmd) = commands.shared.argument.subcommand {
            sub_cmd
        } else {
            let default_name = names::Generator::with_naming(names::Name::Plain)
                .next()
                .unwrap();

            self.query_input("Enter a name for your database", Some(default_name))?
        };

        let database_url = self.query_input("Enter the database url", None)?;

        let db_type = self.query_multi_select(
            vec!["Postgres", "Sqlite", "MySQL", "MongoDB"],
            "Select the database type",
        )?;

        Ok(String::new())
    }
}

use async_trait::async_trait;

use crate::shared::Shared;

use self::new::New;
mod new;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Commands {
    pub shared: Shared,
}

impl Commands {
    pub fn new(shared: Shared) -> Self {
        Self { shared }
    }

    pub async fn exec(&self) -> Result<()> {
        let Shared { argument, prisma } = &self.shared;

        match argument.command.as_str() {
            "new" => {
                let new = New::new();

                new.exec(self.clone()).await?;
            }
            _ => {}
        };

        Ok(())
    }
}

#[async_trait]
pub trait Command {
    type Output;

    async fn exec(&self, commands: Commands) -> Result<Self::Output>;
}

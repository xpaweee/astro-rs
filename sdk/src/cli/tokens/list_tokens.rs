use async_trait::async_trait;
use prettytable::{Table, row, Cell, Row};
use prettytable::format;
use crate::cli::astro_command::AstroCommand;
use crate::errors::AstroError;
use crate::persistence::{cfg_accessor, token_manager};

pub struct ListTokens { }

impl ListTokens
{
    pub fn new() -> Self
    {
        Self { }
    }
}

#[async_trait]
impl AstroCommand for ListTokens
{
    async fn execute(&self) -> Result<(), AstroError> {
        let config = cfg_accessor::ConfigAccessor::new();

        let configs = config.get_configuration_from_file()?;

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.add_row(row!["NAME", "TOKEN", "PROVIDER"]);

        for config in configs {
            table.add_row(Row::new(vec![
                Cell::new(&config.name),
                Cell::new(&config.token),
                Cell::new(&config.provider.to_string()),
            ]));
        }

        table.printstd();

        Ok(())
    }
}
use async_trait::async_trait;
use prettytable::{Table, row, Cell, Row};
use prettytable::format;
use crate::cli::astro_command::AstroCommand;
use integrations::gitlab_client::get_runners;
use crate::errors::AstroError;

pub struct GetRunners { }

#[async_trait]
impl AstroCommand for GetRunners {
    async fn execute(&self) -> Result<(), AstroError> {

        let runners = get_runners().await.unwrap();

        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.add_row(row!["ID", "NAME", "ONLINE", "STATUS", "IP", "DESCRIPTION" ]);

        for runner in runners {
            table.add_row(Row::new(vec![
                Cell::new(&runner.id.to_string()),
                Cell::new(runner.name.as_deref().unwrap_or("")),
                Cell::new(&runner.online.to_string()),
                Cell::new(&runner.status.to_string()),
                Cell::new(&runner.ip_address.to_string()),
                Cell::new(runner.description.as_deref().unwrap_or(""))
            ]));
        }

        table.printstd();

        Ok(())
    }
}


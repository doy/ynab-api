mod app;
mod paths;
mod views;
mod ynab;

use snafu::ResultExt;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("failed to get api key: {}", source))]
    GetApiKey { source: crate::paths::Error },

    #[snafu(display("failed to load budget: {}", source))]
    LoadBudget { source: crate::ynab::BudgetError },
}

pub type Result<T> = std::result::Result<T, Error>;

fn run() -> Result<()> {
    let key = paths::read_api_key().context(GetApiKey)?;
    let budget = ynab::Budget::new(&key).context(LoadBudget)?;

    let mut app = app::App::new(budget);
    app.run();

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ynab-reimbursements: {}", e);
            std::process::exit(1);
        }
    }
}

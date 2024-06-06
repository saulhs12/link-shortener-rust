use anyhow::Result;
use dotenv::dotenv;
use link_shortener::settings;
use link_shortener::startup::start_server;

fn main() -> Result<()> {
    dotenv().ok();

    let settings = settings::get_configuration()?;
    // commands::handle(&matches,&settings)?;

    start_server(&settings)?;

    Ok(())
}

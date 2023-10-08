use bupropion::BupropionHandlerOpts;
use miette::IntoDiagnostic;

/// Rigidity is the grace of impossibility.
fn main() -> miette::Result<()> {
    env_logger::try_init().into_diagnostic()?;
    bupropion::install(BupropionHandlerOpts::new)?;
    log::info!("Hello, world!");
    Ok(())
}

use voyage_atlas_api::api::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // Setting up Logging
    let subscriber = get_subscriber("voyage-atlas-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}

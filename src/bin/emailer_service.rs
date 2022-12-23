use env_logger::Env;
use ostrich_api::{
    config::Config,
    db_conn::DbConn,
    error::{map_ostrich_error, OstrichErrorType},
    models, services,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("🏛 Booting up the Ostrich Service!");

    let config = Arc::new(Config::new(false));
    let email_client = services::email::get_email_client(config.clone());
    let reqwest_client = Arc::new(reqwest::Client::new());

    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let conn = db_conn.get_conn();

    // hopefully this will keep us around 2 requests per second
    let delay = 700;

    let users = models::user::read(&conn);
    for user in users {
        let emailers =
            services::user::get_emails_for_user_with_tier(&conn, user.id, user.billing_id);

        for emailer in emailers {
            let search_param = &emailer.search_param;
            let to = &emailer.email;

            let body = services::email::get_ostrich_email_body(&emailer);

            log::info!("Running search on {} for {}", search_param, to);

            match services::zillow::get_listing_email_for_search_params(
                config.clone(),
                db_conn.clone(),
                reqwest_client.clone(),
                &emailer,
                body,
                delay,
                Some(1), // this means 1 day on market
            )
            .await
            {
                Err(e) => {
                    let etype = e.etype.clone();
                    map_ostrich_error(e);
                    match etype {
                        OstrichErrorType::ListingResultError => {
                            log::info!("Sending followup email to {}", to);
                            let _ = services::email::send_empty_zillow_listings_email(
                                &email_client,
                                config.clone(),
                                &to,
                                search_param,
                            )
                            .await
                            .map_err(map_ostrich_error);
                        }
                        _ => (),
                    }
                }
                Ok(body) => {
                    let _ = services::email::send_zillow_listings_email(
                        &email_client,
                        config.clone(),
                        &to,
                        &body,
                        search_param,
                    )
                    .await
                    .map_err(map_ostrich_error);
                }
            }
        }
    }

    Ok(())
}

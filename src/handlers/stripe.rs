use crate::{config::Config, db_conn::DbConn, models::user::update_user, services};
use std::sync::Arc;
use stripe::{Client, EventObject, WebhookEvent};

pub async fn handle_webhook(
    config: Arc<Config>,
    webhook_event: WebhookEvent,
    client: Arc<Client>,
    db_conn: Arc<DbConn>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    match webhook_event.data.object {
        EventObject::Subscription(subscription) => {
            let subscription_id = subscription.id;
            let subscription =
                services::stripe::get_subscription_from_id(client.clone(), subscription_id.clone())
                    .await
                    .map_err(map_stripe_err)?;

            let customer = services::stripe::get_customer_from_subscription_id(
                client.clone(),
                subscription.id,
            )
            .await
            .map_err(map_stripe_err)?;

            let product =
                services::stripe::get_product_from_subscription_id(client.clone(), subscription_id)
                    .await
                    .map_err(map_stripe_err)?;

            let billing_id = if subscription.status == stripe::SubscriptionStatus::Active {
                product.name.unwrap()
            } else {
                String::from("")
            };

            let results = update_user(&conn, customer.email.unwrap(), billing_id);
            Ok(warp::reply())
        }
        _ => Ok(warp::reply()),
    }
}

fn map_stripe_err(e: stripe::StripeError) -> warp::Rejection {
    log::error!("{:?}", e);
    warp::reject()
}

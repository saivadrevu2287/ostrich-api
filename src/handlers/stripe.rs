use crate::{config::Config, db_conn::DbConn, models::user, services};
use std::sync::Arc;
use stripe::{Client, EventObject, WebhookEvent};

pub async fn handle_webhook(
    config: Arc<Config>,
    webhook_event: WebhookEvent,
    client: Arc<Client>,
    db_conn: Arc<DbConn>,
    email_client: Arc<sendgrid_async::Client>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    match webhook_event.data.object {
        // if its a Subscription event
        EventObject::Subscription(subscription) => {
            let subscription_id = subscription.id;
            // get the actial object from the id
            let subscription =
                services::stripe::get_subscription_from_id(client.clone(), subscription_id.clone())
                    .await
                    .map_err(map_stripe_err)?;

            // get the customer from the subscription
            let customer = services::stripe::get_customer_from_subscription_id(
                client.clone(),
                subscription.id,
            )
            .await
            .map_err(map_stripe_err)?;

            // get what product the subscription is for
            let product =
                services::stripe::get_product_from_subscription_id(client.clone(), subscription_id)
                    .await
                    .map_err(map_stripe_err)?;

            // map our billing id
            let billing_id = if subscription.status == stripe::SubscriptionStatus::Active {
                product.name.unwrap()
            } else {
                String::from("Tier 0")
            };

            let new_user_email = customer.email.clone().unwrap();

            let fetch_active_user = user::get_user_by_email(&conn, new_user_email.clone());
            let active_user = match fetch_active_user.first() {
                Some(user) => user,
                None => {
                    // send email here?
                    return Err(warp::reject::reject());
                }
            };

            services::email::email_admin_on_tier_change(
                &email_client,
                config,
                new_user_email,
                billing_id.clone(),
                active_user.billing_id.clone(),
            )
            .await;

            let results = user::update_user(&conn, customer.email.unwrap(), billing_id);
            Ok(warp::reply())
        }
        _ => Ok(warp::reply()),
    }
}

fn map_stripe_err(e: stripe::StripeError) -> warp::Rejection {
    log::error!("{:?}", e);
    warp::reject()
}

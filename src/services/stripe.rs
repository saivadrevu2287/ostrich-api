use crate::Config;
use std::sync::Arc;
use stripe::{Client, Customer, Product, Subscription, Webhook, WebhookError, WebhookEvent};
use warp::{filters::BoxedFilter, Filter};

pub async fn with_webhook(
    signature: String,
    payload: String,
    config: Arc<Config>,
) -> Result<(Arc<Config>, WebhookEvent), warp::Rejection> {
    let webhook_event = Webhook::construct_event(
        &payload,
        &signature,
        &config.stripe_api.webhook_signature_secret,
    )
    .map_err(|e: WebhookError| {
        log::error!("{:?}", e);
        warp::reject::reject()
    })?;
    Ok((config, webhook_event))
}

pub fn with_client(client: Arc<Client>) -> BoxedFilter<(Arc<Client>,)> {
    warp::any().map(move || client.clone()).boxed()
}

pub fn get_stripe_client(config: Arc<Config>) -> Client {
    let client = Client::new(config.stripe_api.client_secret.clone());
    client
}

// pub async fn get_customer_from_stripe(client: Arc<Client>, ) -> Result<Subscription, stripe::StripeError> {
//     Subscription::retrieve(
//         client,
//         &stripe::def_id!(Subscription, "evt_1LsMosIDd9tdb2o13jvjkVTq"),
//         &["items", "items.data.price.product", "schedule"]
//     ).await
// }

pub async fn get_customer_from_email(
    client: Arc<Client>,
    email: String,
) -> Result<Vec<Customer>, stripe::StripeError> {
    let customer_options = stripe::ListCustomers {
        created: None,
        email: Some(&email),
        ending_before: None,
        expand: &[],
        limit: None,
        starting_after: None,
        test_clock: None,
    };

    let list = Customer::list(&client, customer_options).await?;

    Ok(list.data)
}

pub async fn get_subscription_from_customer(
    client: Arc<Client>,
    customer_id: stripe::CustomerId,
) -> Result<Vec<Subscription>, stripe::StripeError> {
    let subscription_options = stripe::ListSubscriptions {
        collection_method: None,
        current_period_end: None,
        current_period_start: None,
        created: None,
        customer: Some(customer_id),
        ending_before: None,
        expand: &["data.items.data.price"],
        limit: None,
        starting_after: None,
        test_clock: None,
        price: None,
        status: None,
    };

    let list = Subscription::list(&client, subscription_options).await?;

    Ok(list.data)
}

pub async fn get_subscription_from_id(
    client: Arc<Client>,
    subscription_id: stripe::SubscriptionId,
) -> Result<Subscription, stripe::StripeError> {
    let subscription = Subscription::retrieve(&client, &subscription_id, &[]).await?;

    Ok(subscription)
}

pub async fn get_customer_from_id(
    client: Arc<Client>,
    customer_id: stripe::CustomerId,
) -> Result<Customer, stripe::StripeError> {
    let customer = Customer::retrieve(&client, &customer_id, &[]).await?;

    Ok(customer)
}

pub async fn get_product_from_id(
    client: Arc<Client>,
    product_id: stripe::ProductId,
) -> Result<Product, stripe::StripeError> {
    let product = Product::retrieve(&client, &product_id, &[]).await?;

    Ok(product)
}

pub async fn get_product_from_subscription_id(
    client: Arc<Client>,
    subscription_id: stripe::SubscriptionId,
) -> Result<Product, stripe::StripeError> {
    let subscription = get_subscription_from_id(client.clone(), subscription_id).await?;
    if subscription.items.data.len() > 0 {
        let product_id = match subscription.items.data[0]
            .price
            .clone()
            .unwrap()
            .product
            .unwrap()
        {
            stripe::Expandable::Id(id) => Some(id),
            stripe::Expandable::Object(_) => None,
        };

        get_product_from_id(client, product_id.unwrap()).await
    } else {
        Err(stripe::StripeError::ClientError(String::from(
            "The shit is busted",
        )))
    }
}

pub async fn get_customer_from_subscription_id(
    client: Arc<Client>,
    subscription_id: stripe::SubscriptionId,
) -> Result<Customer, stripe::StripeError> {
    let subscription = get_subscription_from_id(client.clone(), subscription_id).await?;
    if subscription.items.data.len() > 0 {
        let customer_id = match subscription.customer {
            stripe::Expandable::Id(id) => Some(id),
            stripe::Expandable::Object(_) => None,
        };

        get_customer_from_id(client, customer_id.unwrap()).await
    } else {
        Err(stripe::StripeError::ClientError(String::from(
            "The shit is busted",
        )))
    }
}

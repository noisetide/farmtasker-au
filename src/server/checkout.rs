use crate::stripe_retypes::{DbCheckoutSession, DbCheckoutSessionStatus};
use crate::{stripe_stater, stripe_sync, ShoppingCart, StripeData};
use leptos::*;
use leptos::ServerFnError;
use log::*;
use std::collections::HashMap;

/// Find if there is existing session by id
#[leptos::server(name = CheckoutSessionMatches)]
pub async fn find_checkout_session_matches(
    checkout_sessionid: String,
) -> Result<bool, ServerFnError> {
    use stripe::*;
    let client = Client::new(match std::env::var("STRIPE_KEY") {
        Ok(ok) => ok,
        Err(err) => {
            log::error!("{:#?}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    });

    let stripe_data: StripeData = stripe_stater().await?;

    Ok(stripe_data.checkout_sessions.iter().any(|session| {
        session.id == checkout_sessionid
            && session
                .status
                .clone()
                .map_or(false, |status| status == DbCheckoutSessionStatus::Open)
    }))
}

/// Creates new checkout session via stripe API using shopping cart items from client
#[leptos::server(name = NewCheckoutSession)]
pub async fn new_checkout_session(
    shopping_cart: HashMap<String, u8>, // shopping_cart input from storage
    checkout_sessionid: String,         // browser checkout_sessionid input from storage
) -> Result<DbCheckoutSession, ServerFnError> {
    let mut cart = ShoppingCart::default();
    cart.0 = shopping_cart;
    let shopping_cart = cart;

    use stripe::*;
    let client = Client::new(match std::env::var("STRIPE_KEY") {
        Ok(ok) => ok,
        Err(err) => {
            log::error!("{:#?}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    });
    let stripe_data: StripeData = stripe_stater().await?;

    let base_url = match std::env::var("DEVPORT") {
        Ok(port) => "http://localhost:4444",
        Err(_) => "https://farmtasker.au",
    };

    let cancel_url = format!("{:#}/shop/cart", base_url);
    let success_url = format!("{:#}/success", base_url);

    let mut params = stripe::CreateCheckoutSession::new();
    params.cancel_url = Some(&cancel_url);
    params.success_url = Some(&success_url);
    params.customer = None;
    params.customer_creation = Some(stripe::CheckoutSessionCustomerCreation::IfRequired);
    params.shipping_address_collection =
        Some(stripe::CreateCheckoutSessionShippingAddressCollection {
            allowed_countries: vec![
                stripe::CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Au,
            ],
        });

    let total_price: i64 = shopping_cart.calculate_total_price(&stripe_data.products);

    let is_cart_under: bool = total_price < 30000;

    params.shipping_options = if is_cart_under {
        Some(vec![CreateCheckoutSessionShippingOptions {
            /// The ID of the Shipping Rate to use for this shipping option.
            shipping_rate: Some(stripe_data.default_shipping_rate_id),

            /// Parameters to be passed to Shipping Rate creation for this shipping option.
            shipping_rate_data: None,
        }])
    } else {
        Some(vec![CreateCheckoutSessionShippingOptions {
            /// The ID of the Shipping Rate to use for this shipping option.
            shipping_rate: Some(stripe_data.free_shipping_rate_id),

            /// Parameters to be passed to Shipping Rate creation for this shipping option.
            shipping_rate_data: None,
        }])
    };
    params.consent_collection = Some(CreateCheckoutSessionConsentCollection {
        payment_method_reuse_agreement: Some(CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement {
            position: CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition::Hidden,
        }),
        ..Default::default()
    });

    // Collect additional information from your customer using custom fields.
    //
    // Up to 3 fields are supported.
    // params.custom_fields = Some(vec![CreateCheckoutSessionCustomFields {
    //     // Configuration for `type=dropdown` fields.
    //     dropdown: Some(CreateCheckoutSessionCustomFieldsDropdown {
    //         options: vec![CreateCheckoutSessionCustomFieldsDropdownOptions {
    //             // The label for the option, displayed to the customer.
    //             //
    //             // Up to 100 characters.
    //             label: String::from("I understand that I live within the delivery route "),

    //             // The value for this option, not displayed to the customer,
    //             // used by your integration to reconcile the option selected by the customer.
    //             //
    //             // Must be unique to this option, alphanumeric, and up to 100 characters.
    //             value: String::from("deliverycollection"),
    //         }],
    //     }),
    //     // dropdown: None,

    //     // The label for the field, displayed to the customer.
    //     label: CreateCheckoutSessionCustomFieldsLabel {
    //         custom: String::from("How to collect your order?"),
    //         type_: CreateCheckoutSessionCustomFieldsLabelType::Custom,
    //     },

    //     // Configuration for `type=numeric` fields.
    //     // numeric: Some(CreateCheckoutSessionCustomFieldsNumeric {
    //     //     ..Default::default()
    //     // }),
    //     numeric: None,

    //     // Whether the customer is required to complete the field before completing the Checkout Session.
    //     //
    //     // Defaults to `false`.
    //     optional: Some(false),

    //     // Configuration for `type=text` fields.
    //     // text: Some(CreateCheckoutSessionCustomFieldsText {}),
    //     text: None,

    //     // The type of the field.
    //     type_: CreateCheckoutSessionCustomFieldsType::Dropdown,

    //     // String of your choice that your integration can use to reconcile this field.
    //     //
    //     // Must be unique to this field, alphanumeric, and up to 200 characters.
    //     key: String::from("deliveryconsent"),
    //     // ..Default::default()
    // }]);

    params.custom_text = Some(CreateCheckoutSessionCustomText {
        shipping_address: Some(CreateCheckoutSessionCustomTextShippingAddress {
            message: "We make deliveries only within Tasmania Derwent Valley or Hobart area."
                .to_string(),
        }),
        after_submit: Some(CreateCheckoutSessionCustomTextAfterSubmit {
            message: "We make deliveries only within Tasmania Derwent Valley or Hobart area."
                .to_string(),
        }),
        ..Default::default()
    });
    params.phone_number_collection =
        Some(stripe::CreateCheckoutSessionPhoneNumberCollection { enabled: true });
    params.ui_mode = Some(stripe::CheckoutSessionUiMode::Hosted);
    params.mode = Some(stripe::CheckoutSessionMode::Payment);
    params.billing_address_collection =
        Some(stripe::CheckoutSessionBillingAddressCollection::Required);
    params.currency = Some(stripe::Currency::AUD);

    let mut line_items_vec = Vec::new();

    if shopping_cart.0.is_empty().clone() {
        error!("NO ITEMS in ShoppingCart. Couldn't create line_items");
        return Err(leptos::ServerFnError::ServerError(
            "NO ITEMS in ShoppingCart. Couldn't create line_items".into(),
        ));
    }

    for (product_id, quantity) in &shopping_cart.0 {
        if let Some(product) = stripe_data.products.iter().find(|p| p.id == *product_id) {
            let line_item = CreateCheckoutSessionLineItems {
                adjustable_quantity: Some(CreateCheckoutSessionLineItemsAdjustableQuantity {
                    enabled: true,
                    maximum: Some(20),
                    minimum: Some(1),
                }),
                quantity: Some((*quantity).into()),
                price: Some(product.default_price.clone().expect("NO PRICE!").id),
                ..Default::default()
            };
            line_items_vec.push(line_item);
        } else {
            error!("NO products in StripeData. Couldn't create line_items");
            return Err(leptos::ServerFnError::ServerError(
                "NO products in StripeData. Couldn't create line_items".into(),
            ));
        }
    }
    params.line_items = Some(line_items_vec);
    params.expand = &["line_items", "line_items.data.price.product"];

    let new_session = stripe::CheckoutSession::create(&client, params).await?;

    info!(
        "Created NEW checkout session: {:#?}, for {:#?} $AUD. (Created: {:#?} / Expires at: {:#?} )",
        &new_session.id,
        new_session.amount_total.unwrap_or(0).clone() as f64 / 100.0,
        &new_session.created,
        &new_session.expires_at
    );

    stripe_sync();

    leptos_axum::redirect(match &new_session.url.clone() {
        Some(url) => url,
        None => "/cancel",
    });

    Ok(new_session.into())
}

#[server (
    name = RedirectToUrl,
)]
pub async fn redirect_to_url(url: String) -> Result<(), leptos::ServerFnError> {
    leptos_axum::redirect(&url);
    Ok(())
}

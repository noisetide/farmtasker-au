use super::{AppStateDataRes, CheckoutSessionIdRes, CheckoutSessionUpdateRes, StripeDataRes};
use crate::*;
use leptos::*;

#[component]
pub fn SuccessCheckout() -> impl IntoView {
    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);

    view! {
        <p>
            "Checkout session with id"
            {checkout_sessionid()}
            " completed successfully!"
        </p>
        <p>
            "You should find details of your order in your email soon."
        </p>
    }
}

#[component]
pub fn CancelCheckout() -> impl IntoView {
    let stripe_data = expect_context::<StripeDataRes>();
    provide_context(stripe_data);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);

    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);
    let set_checkout_sessionid = expect_context::<WriteSignal<CheckoutSessionIdRes>>();
    provide_context(set_checkout_sessionid);

    let submit_checkout = expect_context::<ReadSignal<CheckoutSessionUpdateRes>>();
    provide_context(submit_checkout);
    let set_submit_checkout = expect_context::<WriteSignal<CheckoutSessionUpdateRes>>();
    provide_context(set_submit_checkout);

    // let checkout_session = expect_context::<CheckoutSessionRes>();
    // provide_context(checkout_session);

    // let loading = checkout_session.loading();
    // let is_loading = move || {
    //     if loading() {
    //         "Loading..."
    //     } else {
    //         let session = checkout_session
    //             .get()
    //             .map(|value| {
    //                 value
    //                     .map(|value2| {
    //                         set_checkout_sessionid.update(|s| *s = value2.id.clone());
    //                         value2.id
    //                     })
    //                     .unwrap_or_else(|x| "Loading 2".into())
    //             })
    //             .unwrap_or_else(|| "Loading".into());
    //         // leptos::logging::log!("session: {:#?}", session);
    //         // stripe_data.get().expect("no stripdata lol")
    //         "Checkout"
    //     }
    // };

    view! {
        <div>
            "Checkout Cancelled..."
        </div>
        // <div>
        //     "Checkout Session Id: "
        //     {move || {checkout_sessionid.get()}}
        // </div>
        // <Show
        //     when=move || {
        //         let stripe_data = match stripe_data.get()  {
        //             Some(ok) => ok,
        //             None => return false,
        //         }.unwrap();
        //         stripe_data.checkout_sessions.iter().any(|session| session.id == checkout_sessionid.get())
        //     }
        //     fallback=move || view!{}
        // >
        //     <button on:click=move |_| {
        //         let stripe_data = stripe_data.get().expect("No StripeData!").unwrap();

        //         let mut url = String::new();

        //         if let Some(session) = stripe_data.checkout_sessions.iter().find(|session| session.id == checkout_sessionid.get()) {
        //             url = session.url.to_owned().expect("Checkout session has no url!!!");
        //             spawn_local(async move {
        //                 redirect_to_url(url).await;
        //             })
        //         } else {
        //             leptos::logging::log!("No active checkout session.")
        //         }
        //     }>
        //         "Back to checkout session"
        //     </button>
        // </Show>
    }
}

#[component]
pub fn ShoppingCartPage() -> impl IntoView {
    let stripe_data = expect_context::<StripeDataRes>();
    provide_context(stripe_data);

    let app_state = expect_context::<AppStateDataRes>();
    provide_context(app_state);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);

    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);
    let set_checkout_sessionid = expect_context::<WriteSignal<CheckoutSessionIdRes>>();
    provide_context(set_checkout_sessionid);

    let submit_checkout = expect_context::<ReadSignal<CheckoutSessionUpdateRes>>();
    provide_context(submit_checkout);
    let set_submit_checkout = expect_context::<WriteSignal<CheckoutSessionUpdateRes>>();
    provide_context(set_submit_checkout);

    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);

    // let checkout_session = expect_context::<CheckoutSessionRes>();
    // provide_context(checkout_session);

    // let checkout_loading = checkout_session.loading();
    // let is_checkout_loading = move || {
    //     if checkout_loading() {
    //         "Loading..."
    //     } else {
    //         let session = checkout_session
    //             .get()
    //             .map(|value| {
    //                 value
    //                     .map(|value2| {
    //                         set_checkout_sessionid.update(|s| *s = value2.id.clone());
    //                         value2.id
    //                     })
    //                     .unwrap_or_else(|x| "Loading 2".into())
    //             })
    //             .unwrap_or_else(|| "Loading".into());
    //         // leptos::logging::log!("session: {:#?}", session);
    //         // stripe_data.get().expect("no stripdata lol")
    //         "Checkout"
    //     }
    // };

    view! {
        <Show
            when=move || { shopping_cart.get().0.len() != 0 }
            fallback=|| view!{
                <div>
                    <h3>
                        "Your Shopping Cart is Empty."
                    </h3>
                    <p>
                        "You can browse items in:"
                    </p>
                    <a href="/shop/food">"Farm Food Shop "</a>
                    "or "
                    <a href="/shop/pet">"Pet Food Shop"</a>
                </div>
            }
        >
                <ul class="shopping-list-ul">
                    {
                        shopping_cart.get().0.into_iter()
                            .map(|(product_id, quantity)| {
                                let (product_id, _) = create_signal(product_id.clone());
                                provide_context(product_id);

                                let product_name = if let Some(product) = stripe_data
                                    .get()
                                    .unwrap()
                                    .unwrap()
                                    .products
                                    .into_iter()
                                    .find(|x| x.id == product_id.get())
                                {
                                    create_signal(product.name).0
                                } else {
                                    set_shopping_cart.update(|s| s.remove_single_product(&product_id.get()));
                                    create_signal("Product".to_string()).0
                                };

                                provide_context(product_name);
                                view! {
                                    <li>
                                        <p>
                                            {product_name.get()}", quantity: "{quantity}
                                        </p>
                                        <div>
                                            <Show
                                                when=move || {quantity < 20}
                                                fallback=move || view! {
                                                    <button class="plus_one_product_amount">
                                                    "MAX"
                                                    </button>
                                                }
                                            >
                                                <button class="plus_one_product_amount" on:click=move |_| {
                                                    set_shopping_cart.update(|s| {
                                                        s.add_single_product(&product_id.get(), 20);
                                                    });
                                                }>
                                                "+"
                                                </button>
                                            </Show>
                                            <button class="minus_one_product_amount" on:click=move |_| {
                                                set_shopping_cart.update(|s| {
                                                    s.remove_single_product(&product_id.get());
                                                });
                                            }>
                                                {move || if quantity > 1 {
                                                    "-"
                                                } else {
                                                    "Delete"
                                                }}
                                            </button>
                                        </div>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
                <div class="shopping-cart-ceckout-section">
                    <button class="checkout-button" on:click=move |_| {
                            let checkout_sessionid_before = checkout_sessionid.get();

                            spawn_local(async move {
                                stripe_sync().await;

                                new_checkout_session(shopping_cart.get().0, checkout_sessionid.get()).await;
                            });

                        }>
                        // {is_checkout_loading} // checkout button text
                        "Checkout"
                    </button>
                    <button on:click=move |_| {
                            set_shopping_cart.update(|s| {
                                *s = ShoppingCart::default();
                            });
                        }>
                        "Clear"
                    </button>
                    // <Show
                    //     when=move || {
                    //         let stripe_data = match stripe_data.get()  {
                    //             Some(ok) => ok,
                    //             None => return false,
                    //         }.unwrap();
                    //         stripe_data.checkout_sessions.iter().any(|session| session.id == checkout_sessionid.get())
                    //     }
                    //     fallback=move || view!{}
                    // >
                    //     <button on:click=move |_| {
                    //         let stripe_data = stripe_data.get().expect("No StripeData!").unwrap();

                    //         let mut url = String::new();

                    //         if let Some(session) = stripe_data.checkout_sessions.iter().find(|session| session.id == checkout_sessionid.get()) {
                    //             url = session.url.to_owned().expect("Checkout session has no url!!!");
                    //             spawn_local(async move {
                    //                 redirect_to_url(url).await;
                    //             })
                    //         } else {
                    //             leptos::logging::log!("No active checkout session.")
                    //         }
                    //     }>
                    //         "Back to checkout"
                    //     </button>
                    // </Show>
                </div>
        </Show>
    }
}

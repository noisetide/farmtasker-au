#![allow(unused)]
use crate::error_template::{AppError, ErrorTemplate};

use crate::*;
use leptos::*;
use leptos_dom::logging;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::*;
use leptos_use::*;
use log::*;

pub type StripeDataRes = Resource<(), Result<StripeData, ServerFnError>>;
pub type CheckoutSessionRes = Resource<i64, Result<DbCheckoutSession, ServerFnError>>;

pub type CheckoutSessionIdRes = String;

pub type CheckoutSessionUpdateRes = i64;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let stripe_data: StripeDataRes = create_resource(|| (), move |_| async { stater().await });
    provide_context(stripe_data);

    let (current_page, set_current_page) = create_signal(CurrentPage::None);
    provide_context(current_page);
    provide_context(set_current_page);

    let (shopping_cart, set_shopping_cart, clear_shopping_cart) =
        use_local_storage_with_options::<ShoppingCart, codee::string::JsonSerdeCodec>(
            "shopping_cart",
            UseStorageOptions::default().delay_during_hydration(true),
        );
    provide_context(shopping_cart);
    provide_context(set_shopping_cart);
    provide_context(clear_shopping_cart);

    let (checkout_sessionid, set_checkout_sessionid, clear_checkout_sessionid) =
        use_local_storage_with_options::<CheckoutSessionIdRes, codee::string::JsonSerdeCodec>(
            "checkout_sessionid",
            UseStorageOptions::default().delay_during_hydration(true),
        );
    provide_context(checkout_sessionid);
    provide_context(set_checkout_sessionid);
    provide_context(clear_checkout_sessionid);

    let (submit_checkout, set_submit_checkout): (
        ReadSignal<CheckoutSessionUpdateRes>,
        WriteSignal<CheckoutSessionUpdateRes>,
    ) = create_signal::<CheckoutSessionUpdateRes>(0);
    provide_context(submit_checkout);
    provide_context(set_submit_checkout);

    let checkout_session: CheckoutSessionRes =
        create_resource(submit_checkout, move |x| async move {
            new_checkout_session(shopping_cart.get().clone().0, checkout_sessionid.get()).await
        });
    provide_context(checkout_session);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/farmtasker-au.css"/>

        // sets the document title
        <Title text="Farmtasker Shop"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <nav>
                <NavBar/>
            </nav>
            <main>
                <Routerer/>
            </main>
            <div>
                <FooterBar/>
            </div>
        </Router>
    }
}
#[component]
pub fn Routerer() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::HomePage;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=HomePage currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/pet" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::PetShop;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=PetShop currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/food" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::FoodShop;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=FoodShop currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/products/:product_name"  view={
                move || {
                    let CURRENTPAGE: CurrentPage = CurrentPage::ProductItemDetails;

                    let params = use_params_map();
                    let product_name = params.with(|params| params.get("product_name").cloned()).unwrap_or("no parameter".into());


                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=move || {view!{<ProductItemDetailsPage product_name=product_name.clone()/>}} currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/about" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::About;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=About currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/privacy" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::PrivacyPolicy;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=PrivacyPolicy currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/terms" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::None;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::TermsOfService);
                    view! {
                        <Pager page=TermsOfService currentpage=CurrentPage::TermsOfService/>
                    }
                }
            }/>
            <Route path="/shop/cart" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::ShoppingCart;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=ShoppingCart currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/video/instructions" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::VideoInstructionsService;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=VideoInstructionsService currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/video/blog/culinary-adventure" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::VideoBlogCulinaryAdventure;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=VideoBlogCulinaryAdventure currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/success" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::None;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=SuccessCheckout currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/cancel" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::None;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=CancelCheckout currentpage=CURRENTPAGE/>
                    }
                }
            }/>
        </Routes>
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CurrentPage {
    None,
    HomePage,
    PetShop,
    FoodShop,
    About,
    ProductItemDetails,
    PrivacyPolicy,
    TermsOfService,
    ShoppingCart,
    VideoInstructionsService,
    VideoBlogCulinaryAdventure,
}

#[component]
pub fn Pager<F, IV>(page: F, currentpage: CurrentPage) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div class="page">
            <div class="pager-bg">
                <div class="pager">
                    <div
                        class="pager-content"
                        class=match currentpage {
                            CurrentPage::None => {"pager-content-none"},
                            CurrentPage::HomePage => {"pager-content-home-page"},
                            CurrentPage::PetShop => {"pager-content-pet-shop pager-content-shop-general"},
                            CurrentPage::FoodShop => {"pager-content-foot-shop pager-content-shop-general"},
                            CurrentPage::About => {"pager-content-about"},
                            CurrentPage::PrivacyPolicy => {"pager-content-privacy-policy"},
                            CurrentPage::TermsOfService => {"pager-content-terms-of-service"},
                            CurrentPage::ShoppingCart => {"pager-content-shopping-cart"},
                            CurrentPage::VideoInstructionsService => {"pager-content-video-instructions-service"},
                            CurrentPage::VideoBlogCulinaryAdventure => {"pager-content-video-blog-culinary-adventure"},
                            CurrentPage::ProductItemDetails => {"pager-content-product-item-details"},
                        }
                    >{page()}</div>
                </div>
            </div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <a href="/shop/food" class="shop-selector-container" id="button_farm_to_table_near_me">
            <p class="shop_selector_title">"Online Shop"</p>
        </a>
        <a href="/shop/pet" class="shop-selector-container" id="button_farmtasker_pet_food_shop">
            <p class="shop_selector_title">"Online Shop"</p>
        </a>
        <a href="/video/instructions" class="shop-selector-container" id="button_farm_task_video_instructions_service">
            <p class="shop_selector_title">"Video Instructions"</p>
        </a>
        <a href="/video/blog/culinary-adventure" class="shop-selector-container" id="button_culinary_adventure">
            <p class="shop_selector_title">"Video Blog"</p>
        </a>
    }
}

#[component]
pub fn ProductItemDetails(product: DbProduct) -> impl IntoView {
    let (product, _) = create_signal(product);
    provide_context(product);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    view! {
        <div class="product-item-container">
            <Show
                when=move || {product.get().images.is_some_and(|x| !x.is_empty())}
                fallback=move || {view!{
                    <div class="product-item-empty">
                    </div>
                }}
            >
                <img class="product-item-image" src={product.get().images.unwrap().first()}/>
            </Show>
            <div class="product-info">
                <strong class="product-item-name">
                    {product.get().name}
                </strong>
                <p class="product-item-description">
                    {product.get().description.unwrap_or("No Description.".to_string())}
                </p>
            </div>
            <button class="product-item-addtocart-button" on:click=move |_| {
                set_shopping_cart.update(|s| {
                    s.add_single_product(&product.get().id, 20);
                });
            }>
            "Add To Cart $"{product.get().default_price.unwrap().unit_amount.unwrap() / 100}
            </button>
        </div>
    }
}

#[component]
pub fn ProductItemDetailsPage(product_name: String) -> impl IntoView {
    let stripe_data = expect_context::<StripeDataRes>();
    let (product_name, _) = create_signal(product_name);
    provide_context(product_name);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    view! {
        <Suspense fallback=move || view! {"loading data"}>
            {move || match stripe_data.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(stripe_data) => {
                    let stripe_data: StripeData = stripe_data.expect("Resource StripeData is not here on 'get()'");
                    let product_name = expect_context::<ReadSignal<String>>();
                    provide_context(product_name);


                    match stripe_data.products.into_iter()
                    .find(|product| {
                        let cmp1 = product.name.to_lowercase().replace(" ", "-");
                        let cmp2 = &product_name.get()[1..];

                        cmp1 == cmp2
                    }) {
                        Some(product) => {
                            view!{
                                <ProductItemDetails product=product/>
                            }.into_view()
                        },
                        None => view!{
                            <div>"NO PRODUCT WITH SUCH NAME"</div>
                        }.into_view(),
                    }
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn PetShop() -> impl IntoView {
    view! {
        <h1 class="shop-title">"Pet Food Shop"</h1>
        <ProductItemsList items_category="pet_food".to_string()/>
    }
}

#[component]
pub fn FoodShop() -> impl IntoView {
    view! {
        <h1 class="shop-title">"Farm Food Shop"</h1>
        <ProductItemsList items_category="food".to_string()/>
    }
}

#[component]
pub fn VideoInstructionsService() -> impl IntoView {
    view! {
        <div>"Video Instructions Service!!!"</div>
    }
}

#[component]
pub fn VideoBlogCulinaryAdventure() -> impl IntoView {
    view! {
        <div>"Video Blog Culinary Adventure!!!"</div>
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="about-us-image-container">
            <img class="about-us-image" src="/photos/DSCF6711.jpg" alt="About Us"/>
            <strong class="about-us-image-block-1">
                "About"
            </strong>
            <strong class="about-us-image-block-2">
                "Us"
            </strong>
        </div>
    }
}

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    view! {
        <div class="privacy-policy">
            <h1>"Privacy Policy"</h1>
            <p>
                "Our Privacy Policy is currently being prepared and will be available soon. If you have any questions, please contact us at "
                <a href="mailto:info@farmtasker.au">" info@farmtasker.au"</a>
                "."
            </p>
        </div>
    }
}

#[component]
pub fn TermsOfService() -> impl IntoView {
    view! {
        <div class="terms-of-service">
            <h1>"Terms of Service"</h1>
            <p>
                "Our Terms of Service are currently being prepared and will be available soon. If you have any questions, please contact us at "
                <a href="mailto:info@farmtasker.au">" info@farmtasker.au"</a>
                "."
            </p>
        </div>
    }
}
#[component]
pub fn ProductItem(product: DbProduct) -> impl IntoView {
    let (product, _) = create_signal(product);
    provide_context(product);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    view! {
        <div class="product-item-container">
            <a href=move || {
                let product_name = product.get().name.to_lowercase().replace(" ", "-");
                format!("/shop/products/:{:#}", product_name)
            }>
                <Show
                    when=move || {product.get().images.is_some_and(|x| !x.is_empty())}
                    fallback=move || {view!{
                        <div class="product-item-empty">
                        </div>
                    }}
                >
                    <img class="product-item-image" src={product.get().images.unwrap().first()}/>
                </Show>
                <div class="product-info">
                    <strong class="product-item-name">
                        {product.get().name}
                    </strong>
                </div>
            </a>
            <button class="product-item-addtocart-button" on:click=move |_| {
                set_shopping_cart.update(|s| {
                    s.add_single_product(&product.get().id, 20);
                });
            }>
            "Add To Cart $"{product.get().default_price.unwrap().unit_amount.unwrap() / 100}
            </button>
        </div>
    }
}

#[component]
pub fn ProductItemsList(items_category: String) -> impl IntoView {
    let stripe_data = expect_context::<StripeDataRes>();
    let (items_category, set_items_category) = create_signal(items_category);
    provide_context(items_category);

    view! {
        <Suspense fallback=move || view! {"loading data"}>
            {move || match stripe_data.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(stripe_data) => {
                    let stripe_data: StripeData = stripe_data.expect("Resource StripeData is not here on 'get()'");
                    let items_category = expect_context::<ReadSignal<String>>();
                    provide_context(items_category);


                    view! {
                        <ul class="product-list-ul">
                            {
                                stripe_data.products.into_iter()
                                .filter(|product| {
                                    product.metadata
                                        .as_ref()
                                        .and_then(|metadata| metadata.get("category"))
                                        .map(|category| category == &items_category.get())
                                        .unwrap_or(false)
                                })
                                .map(|product| {
                                    view! {
                                        <li class="product-list-item">
                                            <ProductItem product=product/>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    }.into_view()
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn SuccessCheckout() -> impl IntoView {
    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);

    view! {
        <p>
            "Checkout Completed!"
        </p>
        <p>
            "You will find details of your order in your email soon."
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

    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);

    let checkout_session = expect_context::<CheckoutSessionRes>();
    provide_context(checkout_session);

    let loading = checkout_session.loading();
    let is_loading = move || {
        if loading() {
            "Loading..."
        } else {
            let session = checkout_session
                .get()
                .map(|value| {
                    value
                        .map(|value2| {
                            set_checkout_sessionid.update(|s| *s = value2.id.clone());
                            value2.id
                        })
                        .unwrap_or_else(|x| "Loading 2".into())
                })
                .unwrap_or_else(|| "Loading".into());
            // leptos::logging::log!("session: {:#?}", session);
            // stripe_data.get().expect("no stripdata lol")
            "Checkout"
        }
    };

    view! {
        <div>
            "Checkout Cancelled..."
        </div>
        <Show
            when=move || {
                let stripe_data = match stripe_data.get()  {
                    Some(ok) => ok,
                    None => return false,
                }.unwrap();
                stripe_data.checkout_sessions.iter().any(|session| session.id == checkout_sessionid.get())
            }
            fallback=move || view!{}
        >
            <button on:click=move |_| {
                let stripe_data = stripe_data.get().expect("No StripeData!").unwrap();

                let mut url = String::new();

                if let Some(session) = stripe_data.checkout_sessions.iter().find(|session| session.id == checkout_sessionid.get()) {
                    url = session.url.to_owned().expect("Checkout session has no url!!!");
                    spawn_local(async move {
                        redirect_to_url(url).await;
                    })
                } else {
                    leptos::logging::log!("No active checkout session.")
                }
            }>
                "Back to checkout"
            </button>
        </Show>

    }
}

#[component]
pub fn ShoppingCart() -> impl IntoView {
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

    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);

    let checkout_session = expect_context::<CheckoutSessionRes>();
    provide_context(checkout_session);

    let loading = checkout_session.loading();
    let is_checkout_loading = move || {
        if loading() {
            "Loading..."
        } else {
            let session = checkout_session
                .get()
                .map(|value| {
                    value
                        .map(|value2| {
                            set_checkout_sessionid.update(|s| *s = value2.id.clone());
                            value2.id
                        })
                        .unwrap_or_else(|x| "Loading 2".into())
                })
                .unwrap_or_else(|| "Loading".into());
            // leptos::logging::log!("session: {:#?}", session);
            // stripe_data.get().expect("no stripdata lol")
            "Checkout"
        }
    };

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
                    <a href="/shop/food">"Food Shop "</a>
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

                                let (product_name, _) = create_signal(stripe_data.get().unwrap().unwrap().products.into_iter().find(|x| x.id == product_id.get()).unwrap().name);
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

                            set_submit_checkout.update(|s| {
                                *s += 1;
                            });

                            spawn_local(async {
                                stripe_sync().await;
                            });

                        }>
                        {is_checkout_loading} // checkout button text
                    </button>
                    <button on:click=move |_| {
                            set_shopping_cart.update(|s| {
                                *s = ShoppingCart::default();
                            });
                        }>
                        "Clear"
                    </button>
                    <Show
                        when=move || {
                            let stripe_data = match stripe_data.get()  {
                                Some(ok) => ok,
                                None => return false,
                            }.unwrap();
                            stripe_data.checkout_sessions.iter().any(|session| session.id == checkout_sessionid.get())
                        }
                        fallback=move || view!{}
                    >
                        <button on:click=move |_| {
                            let stripe_data = stripe_data.get().expect("No StripeData!").unwrap();

                            let mut url = String::new();

                            if let Some(session) = stripe_data.checkout_sessions.iter().find(|session| session.id == checkout_sessionid.get()) {
                                url = session.url.to_owned().expect("Checkout session has no url!!!");
                                spawn_local(async move {
                                    redirect_to_url(url).await;
                                })
                            } else {
                                leptos::logging::log!("No active checkout session.")
                            }
                        }>
                            "Back to checkout"
                        </button>
                    </Show>
                </div>
        </Show>
    }
}

#[component]
pub fn FooterBar() -> impl IntoView {
    view! {
        <footer class="footerbar">
            <div class="footer-content">
                <div class="footer-section">
                    <p>"© 2024 FARMTASKER PTY LTD. All rights reserved."</p>
                </div>
                <div class="footer-section">
                    <p>
                        "Contact us: "
                        <a href="mailto:info@farmtasker.au">"info@farmtasker.au"</a>
                    </p>
                </div>
                // <div class="footer-section">
                //     <p>
                //         <a href="/privacy">"Privacy Policy"</a> |
                //         <a href="/terms">"Terms of Service"</a>
                //     </p>
                // </div>
            </div>
        </footer>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let selected = expect_context::<ReadSignal<CurrentPage>>();

    let (is_navbar_hidden, set_is_navbar_hidden) = create_signal(true);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);

    view! {
        <nav class="navbar">
            <div class="navbar-hide-block">
                <button class="navbar-hide-button"
                    on:click=move |_| {
                        set_is_navbar_hidden.update(|n| *n = !*n);
                    }
                >
                    <div class="bar"></div>
                    <div class="bar"></div>
                    <div class="bar"></div>
                </button>
            </div>
            <div class="shopping-cart-hide-block">
                <a href="/shop/cart" class="navbar-hide-button"
                    class:current=move || {
                        matches!(selected.get(), CurrentPage::ShoppingCart)
                    }
                >
                    "🛒 "{move || match shopping_cart.get().0.values().map(|&v| v as usize).sum() {
                        0 => "".to_string(),
                        x => x.to_string(),
                    }}
                </a>
            </div>
            <div class="banner-bg">
                <div class="logo-container">
                    <a href="/">
                        <img src="/main_logo.svg" alt="Farmtasker Logo" loading="lazy"/>
                    </a>
                </div>
            </div>
            <ul class="nav_buttons" class:is-navbar-hidden=move || is_navbar_hidden()>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::HomePage)}
                        href="/" id="button_middle"
                    >
                        <img
                             src="/buttons/online_shop.png" class="button_middle_image" alt="Home"
                                style:height="auto"
                                style:width="100%"
                                style:max-height="8vh"
                                style:object-fit="contain"
                        />
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::FoodShop)}
                        href="/shop/food" id="button_middle"
                    >
                        <img
                             src="/buttons/food_shop.png" class="button_middle_image" alt="Food Shop"
                                style:height="auto"
                                style:width="100%"
                                style:max-height="8vh"
                                style:object-fit="contain"
                        />
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::PetShop)}
                        href="/shop/pet" id="button_middle"
                    >
                        <img
                             src="/buttons/pet_shop.png" class="button_middle_image" alt="Pet Shop"
                                style:height="auto"
                                style:width="100%"
                                style:max-height="8vh"
                                style:object-fit="contain"
                        />
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::VideoInstructionsService)}
                        href="/video/instructions" id="button_middle"
                    >
                        <img
                             src="/buttons/video_instructions.png" class="button_middle_image" alt="Video Instructions"
                                style:height="auto"
                                style:width="100%"
                                style:max-height="8vh"
                                style:object-fit="contain"
                        />
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::VideoBlogCulinaryAdventure)}
                        href="/video/blog/culinary-adventure" id="button_middle"
                    >
                        <img
                             src="/buttons/video_blog.png" class="button_middle_image" alt="Video Blogs"
                                style:height="auto"
                                style:width="100%"
                                style:max-height="8vh"
                                style:object-fit="contain"
                        />
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::About)}
                        href="/about" id="button_middle"
                    >
                        <img
                             src="/buttons/about_us.png" class="button_middle_image" alt="About Us"
                                style:height="auto"
                                style:width="100%"
                                style:max-height="8vh"
                                style:object-fit="contain"
                        />
                    </a>
                </li>
                // <li>
                //     <a
                //     class:current=move || {
                //         matches!(selected.get(), CurrentPage::ShoppingCart)
                //     }
                //         href="/shop/cart" id="button_middle">"🛒 ""Cart"</a>
                // </li>
            </ul>
        </nav>
    }
}

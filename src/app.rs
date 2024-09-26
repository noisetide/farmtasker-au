#![allow(unused)]
use crate::error_template::{AppError, ErrorTemplate};

use crate::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::*;

pub type StripeDataRes = Resource<(), Result<StripeData, ServerFnError>>;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (current_page, set_current_page) = create_signal(CurrentPage::None);
    provide_context(current_page);
    provide_context(set_current_page);
    let (shopping_cart, set_shopping_cart) = create_signal(ShoppingCart::default());
    provide_context(shopping_cart);
    provide_context(set_shopping_cart);

    let stripe_data: StripeDataRes = create_resource(|| (), move |_| async { stater().await });
    provide_context(stripe_data);

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
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::HomePage);
                    view! {
                        <Pager page=HomePage currentpage=CurrentPage::HomePage/>
                    }
                }
            }/>
            <Route path="/shop/pet" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::PetShop);
                    view! {
                        <Pager page=PetShop currentpage=CurrentPage::PetShop/>
                    }
                }
            }/>
            <Route path="/shop/food" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::FoodShop);
                    view! {
                        <Pager page=FoodShop currentpage=CurrentPage::FoodShop/>
                    }
                }
            }/>
            <Route path="/about" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::About);
                    view! {
                        <Pager page=About currentpage=CurrentPage::About/>
                    }
                }
            }/>
            <Route path="/privacy" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::PrivacyPolicy);
                    view! {
                        <Pager page=PrivacyPolicy currentpage=CurrentPage::PrivacyPolicy/>
                    }
                }
            }/>
            <Route path="/terms" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::TermsOfService);
                    view! {
                        <Pager page=TermsOfService currentpage=CurrentPage::TermsOfService/>
                    }
                }
            }/>
            <Route path="/shop/cart" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::ShoppingCart);
                    view! {
                        <Pager page=ShoppingCart currentpage=CurrentPage::ShoppingCart/>
                    }
                }
            }/>
            <Route path="/video/instructions" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::VideoInstructionsService);
                    view! {
                        <Pager page=VideoInstructionsService currentpage=CurrentPage::VideoInstructionsService/>
                    }
                }
            }/>
            <Route path="/video/blog/culinary-adventure" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::VideoBlogCulinaryAdventure);
                    view! {
                        <Pager page=VideoBlogCulinaryAdventure currentpage=CurrentPage::VideoBlogCulinaryAdventure/>
                    }
                }
            }/>
            <Route path="/success" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::None);
                    view! {
                        <Pager page=SuccessCheckout currentpage=CurrentPage::None/>
                    }
                }
            }/>
            <Route path="/cancel" view={
                move || {
                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::None);
                    view! {
                        <Pager page=CancelCheckout currentpage=CurrentPage::None/>
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
                    <div class="pager-content" class=match currentpage {
                        CurrentPage::None => {"pager-content-none"},
                        CurrentPage::HomePage => {"pager-content-home-page"},
                        CurrentPage::PetShop => {"pager-content-pet-shop"},
                        CurrentPage::FoodShop => {"pager-content-foot-shop"},
                        CurrentPage::About => {"pager-content-about"},
                        CurrentPage::PrivacyPolicy => {"pager-content-privacy-policy"},
                        CurrentPage::TermsOfService => {"pager-content-terms-of-service"},
                        CurrentPage::ShoppingCart => {"pager-content-shopping-cart"},
                        CurrentPage::VideoInstructionsService => {"pager-content-video-instructions-service"},
                        CurrentPage::VideoBlogCulinaryAdventure => {"pager-content-video-blog-culinary-adventure"}
                    }>{page()}</div>
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
pub fn PetShop() -> impl IntoView {
    view! {
        <div>"PetShop!!!"</div>
        <ProductItems items_category="pet_food".to_string()/>
    }
}

#[component]
pub fn FoodShop() -> impl IntoView {
    view! {
        <div>"FoodShop!!!"</div>
        <ProductItems items_category="food".to_string()/>
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
            <div class="about-us-image-block-1">
                "About"
            </div>
            <div class="about-us-image-block-2">
                "Us"
            </div>
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
pub fn ProductItems(items_category: String) -> impl IntoView {
    let stripe_data = expect_context::<StripeDataRes>();
    let (items_category, set_items_category) = create_signal(items_category);
    provide_context(items_category);

    let shopping_cart = expect_context::<ReadSignal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    view! {
        <Suspense fallback=move || view! {"loading data"}>
            {move || match stripe_data.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(stripe_data) => {
                    let stripe_data: StripeData = stripe_data.expect("Resource StripeData is not here on 'get()'");
                    let items_category = expect_context::<ReadSignal<String>>();

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
                                        <li>
                                            <div>
                                                {product.name}, {product.default_price.unwrap().unit_amount.unwrap() / 100}"$ AUD"
                                                <button on:click=move |_| {
                                                    // TODO make sessionned adding to cart
                                                    set_shopping_cart.update(|s| {
                                                        s.add_single_product(product.id.clone());
                                                    });
                                                    // leptos::logging::log!("Added to Cart! {:#?}", product.id);
                                                    // leptos::logging::log!("Shopping Cart: {:#?}", shopping_cart.get());
                                                }>
                                                "Add To Cart"
                                                </button>
                                            </div>
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
    view! {
        <div>
            "Checkout Cancelled..."
        </div>
    }
}

#[component]
pub fn ShoppingCart() -> impl IntoView {
    let shopping_cart = expect_context::<ReadSignal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    // TODO show shopping cart info + add checkout button for creating new checkout session

    view! {{move || {
        if shopping_cart.get().0.len() != 0 {
            view! {
                <ul class="shopping-list-ul">
                    {
                        shopping_cart.get().0.into_iter()
                            .map(|(product_id, quantity)| {
                                let product_name = product_id.clone();
                                view! {
                                    <li>
                                        <p>
                                            {product_name}", quantity: "{quantity}
                                        </p>
                                        <div>
                                            <button on:click=move |_| {
                                                // TODO make sessionned adding to cart
                                                set_shopping_cart.update(|s| {
                                                    s.remove_single_product(product_id.clone());
                                                });
                                                // leptos::logging::log!("Removed From Cart! {:#?}", product_id);
                                                // leptos::logging::log!("Shopping Cart: {:#?}", shopping_cart.get());
                                            }>
                                            "Remove"
                                            </button>
                                        </div>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
                <div class="shopping-cart-ceckout-section">
                    <button on:click=move |_| {
                            spawn_local(async move {
                                let mut cart: ShoppingCart = shopping_cart.get();

                                new_checkout_session(cart.0).await;

                            });
                        }>
                        "Checkout"
                    </button>
                </div>
            }
        } else {
            view! {
                <ul>
                </ul>
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
        }
    }}}
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

    let shopping_cart = expect_context::<ReadSignal<ShoppingCart>>();
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
                    class:current=move || {
                        matches!(selected.get(), CurrentPage::FoodShop)
                    }
                        href="/shop/food" id="button_middle">"Food Shop"</a>
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected.get(), CurrentPage::PetShop)
                    }
                        href="/shop/pet" id="button_middle">"Pet Shop"</a>
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected.get(), CurrentPage::VideoInstructionsService)
                    }
                        href="/video/instructions" id="button_middle">"Video Instructions"</a>
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected.get(), CurrentPage::VideoBlogCulinaryAdventure)
                    }
                        href="/video/blog/culinary-adventure" id="button_middle">"Video Blogs"</a> // TODO GLOBAL BLOGS PAGE
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected.get(), CurrentPage::About)
                    }
                        href="/about" id="button_middle">"About Us"</a>
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

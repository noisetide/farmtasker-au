//                   GNU LESSER GENERAL PUBLIC LICENSE
//                        Version 2.1, February 1999
//
// farmtasker.au a marketplace website for local farmers in Tasmania.
// Copyright (C) 2024 Dmytro Serdiukov & FARMTASKER PTY LTD
//
// NOTE: All images, logos, and branding are the exclusive property of FARMTASKER PTY LTD and are not covered by the open-source license.
// These assets may not be used publically without prior written permission.
//
// This software is a free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//
// See the GNU Lesser General Public License for more details.
//
// FARMTASKER PTY LTD, hereby disclaims all copyright interest in the
// software `farmtasker.au' (a marketplace website for local farmers in Tasmania) written by Dmytro Serdiukov.
//
// You can contact us at farmtasker@gmail.com

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

mod cart;
mod chrome;
mod pages;
mod products;

pub use cart::*;
pub use chrome::*;
pub use pages::*;
pub use products::*;


pub type AppStateDataRes = Resource<(), Result<AppState, ServerFnError>>;
pub type StripeDataRes = Resource<(), Result<StripeData, ServerFnError>>;
// pub type CfgProductsRes = Resource<(), Result<CfgProducts, ServerFnError>>;
// pub type CheckoutSessionRes = Resource<i64, Result<DbCheckoutSession, ServerFnError>>;
pub type CheckoutSessionIdRes = String;
pub type CheckoutSessionUpdateRes = i64;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let stripe_data: StripeDataRes =
        create_resource(|| (), move |_| async { stripe_stater().await });
    provide_context(stripe_data);

    let app_state: AppStateDataRes =
        create_resource(|| (), move |_| async { appstate_stater().await });
    provide_context(app_state);

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

    // let checkout_session: CheckoutSessionRes =
    //     create_resource(submit_checkout, move |x| async move {
    //         new_checkout_session(shopping_cart.get().clone().0, checkout_sessionid.get()).await
    //     });
    // provide_context(checkout_session);

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
            <nav class="navbar_nav">
                <NavBar/>
            </nav>
            <main class="main_main">
                <Routerer/>
            </main>
            <footer class="footerbar_footer">
                <FooterBar/>
            </footer>
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
                    const CURRENTPAGE: CurrentPage = CurrentPage::PetFood;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=PetFood currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/food" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::FarmFood;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=FarmFood currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/products/:product_name"  view={
                move || {
                    let CURRENTPAGE: CurrentPage = CurrentPage::ProductItemDetailsPage;

                    let params = use_params_map();
                    let product_name = params.with(|params| params.get("product_name").cloned()).unwrap_or("no parameter".into());


                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=move || {view!{<CfgProductItemDetailsPage product_name=product_name.clone()/>}} currentpage=CURRENTPAGE/>
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
            <Route path="/delivery" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::Delivery;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=Delivery currentpage=CURRENTPAGE/>
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
                        <Pager page=ShoppingCartPage currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/instructions" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::VideoInstructions;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=VideoInstructions currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/blog/culinary-adventure" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::VideoBlogs;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=VideoBlogs currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/ready-to-eat" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::ReadyToEat;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=ReadyToEat currentpage=CURRENTPAGE/>
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
    PetFood,
    FarmFood,
    ReadyToEat,
    About,
    Delivery,
    ProductItemDetailsPage,
    PrivacyPolicy,
    TermsOfService,
    ShoppingCart,
    VideoInstructions,
    VideoBlogs,
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
                            // Main pages
                            CurrentPage::None => {"pager-content-none"},
                            CurrentPage::HomePage => {"pager-content-home-page"},
                            // Shop pages
                            CurrentPage::PetFood => {"pager-content-pet-shop pager-content-shop-general"},
                            CurrentPage::FarmFood => {"pager-content-food-shop pager-content-shop-general"},
                            CurrentPage::ReadyToEat => {"pager-content-ready-to-eat-shop pager-content-shop-general"},
                            // Other pages
                            CurrentPage::About => {"pager-content-about"},
                            CurrentPage::Delivery => {"pager-content-delivery"},
                            CurrentPage::PrivacyPolicy => {"pager-content-privacy-policy"},
                            CurrentPage::TermsOfService => {"pager-content-terms-of-service"},
                            CurrentPage::ShoppingCart => {"pager-content-shopping-cart"},
                            CurrentPage::VideoInstructions => {"pager-content-video-instructions"},
                            CurrentPage::VideoBlogs => {"pager-content-video-blogs"},
                            CurrentPage::ProductItemDetailsPage => {"pager-content-product-item-details"},
                        }
                    >{page()}</div>
                </div>
            </div>
        </div>
    }
}

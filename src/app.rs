#![allow(unused)]
use crate::error_template::{AppError, ErrorTemplate};

use crate::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (current_page, set_current_page) = create_signal(CurrentPage::None);
    provide_context(current_page);
    provide_context(set_current_page);

    // let state = expect_context::<u64>();

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
                <Routes>
                    <Route path="/" view={
                        move || {
                            let setter = expect_context::<WriteSignal<CurrentPage>>();
                            setter.update(|page: &mut CurrentPage| *page = CurrentPage::HomePage);
                            view! {
                                <Pager page=HomePage/>
                            }
                        }
                    }/>
                    <Route path="/shop/pet" view={
                        move || {
                            let setter = expect_context::<WriteSignal<CurrentPage>>();
                            setter.update(|page: &mut CurrentPage| *page = CurrentPage::PetShop);
                            view! {
                                <Pager page=PetShop/>
                            }
                        }
                    }/>
                    <Route path="/shop/food" view={
                        move || {
                            let setter = expect_context::<WriteSignal<CurrentPage>>();
                            setter.update(|page: &mut CurrentPage| *page = CurrentPage::FoodShop);
                            view! {
                                <Pager page=FoodShop/>
                            }
                        }
                    }/>
                    <Route path="/about" view={
                        move || {
                            let setter = expect_context::<WriteSignal<CurrentPage>>();
                            setter.update(|page: &mut CurrentPage| *page = CurrentPage::About);
                            view! {
                                <Pager page=About/>
                            }
                        }
                    }/>
                    <Route path="/privacy" view={
                        move || {
                            let setter = expect_context::<WriteSignal<CurrentPage>>();
                            setter.update(|page: &mut CurrentPage| *page = CurrentPage::PrivacyPolicy);
                            view! {
                                <Pager page=PrivacyPolicy/>
                            }
                        }
                    }/>
                    <Route path="/terms" view={
                        move || {
                            let setter = expect_context::<WriteSignal<CurrentPage>>();
                            setter.update(|page: &mut CurrentPage| *page = CurrentPage::TermsOfService);
                            view! {
                                <Pager page=TermsOfService/>
                            }
                        }
                    }/>
                    <Route path="/shop/cart" view={
                        move || {
                            let setter = expect_context::<WriteSignal<CurrentPage>>();
                            setter.update(|page: &mut CurrentPage| *page = CurrentPage::ShoppingCart);
                            view! {
                                <Pager page=ShoppingCart/>
                            }
                        }
                    }/>
                </Routes>
            </main>
            <nav>
                <FooterBar/>
            </nav>
        </Router>
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
}

#[component]
pub fn Pager<F, IV>(page: F) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div class="page">
            <div class="pager-bg">
                <div class="pager">
                    <div class="pager-content">{page()}</div>
                </div>
            </div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div id="shop_selector2" class="shop_selector_container">
            <a href="/shop/food">
                <p class="shop_selector_title">"Online Shop"</p>
                <img src="/button_farm_to_table.png" alt="Food Shop"/>
            </a>
        </div>
        <div id="shop_selector1" class="shop_selector_container">
            <a href="/shop/pet">
                <p class="shop_selector_title">"Online Shop"</p>
                <img src="/button_pet_food_shop.png" alt="Pet Shop"/>
            </a>
        </div>
    }
}

#[component]
pub fn PetShop() -> impl IntoView {
    view! {
        <div>"PetShop!!!"</div>
    }
}
#[component]
pub fn FoodShop() -> impl IntoView {
    view! {
        <div>"FoodShop!!!"</div>
    }
}
#[component]
pub fn About() -> impl IntoView {
    view! {
        <div>"About!!!"</div>
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
pub fn ShoppingCart() -> impl IntoView {
    let page = expect_context::<ReadSignal<CurrentPage>>();
    view! {
        <div>
            <strong>{move || format!("{:#?}", page.get())}</strong>
        </div>
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

    view! {
        <nav class="navbar">
            <div class="logo-container">
                <a href="/">
                    <img src="/main_logo.svg" alt="Farmtasker Logo" loading="lazy"/>
                </a>
            </div>
            <h4 class="title-text">"Marketplace for farmers & pet food manufacturers"</h4>
            <ul class="nav_buttons">
                <li>
                    <a class:current=move || {
                        matches!(selected.get(), CurrentPage::HomePage)
                    }
                        href="/" id="button_left">"Home"</a>
                </li>
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
                        matches!(selected.get(), CurrentPage::About)
                    }
                        href="/about" id="button_middle">"About Us"</a>
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected.get(), CurrentPage::ShoppingCart)
                    }
                        href="/shop/cart" id="button_middle">"🛒"</a>
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected.get(), CurrentPage::TermsOfService)
                    }
                        href="/terms" id="button_right">"?"</a>
                </li>
            </ul>
        </nav>
    }
}

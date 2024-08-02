#![allow(unused)]
use crate::error_template::{AppError, ErrorTemplate};
#[cfg(feature = "ssr")]
use axum::Extension;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::*;
use std::sync::{Arc, Mutex};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
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
            <main>
                <Routes>
                    <Route path="/" view=move || view! {
                            <GlobalPage current_page=CurrentPage::HomePage/>
                        }
                    />
                    <Route path="/shop/pet" view=move || view! {
                            <GlobalPage current_page=CurrentPage::PetShop/>
                        }
                    />
                    <Route path="/shop/food" view=move || view! {
                            <GlobalPage current_page=CurrentPage::FoodShop/>
                        }
                    />
                    <Route path="/about" view=move || view! {
                            <GlobalPage current_page=CurrentPage::About/>
                        }
                    />
                    <Route path="/privacy" view=move || view! {
                            <GlobalPage current_page=CurrentPage::PrivacyPolicy/>
                        }
                    />
                    <Route path="/terms" view=move || view! {
                            <GlobalPage current_page=CurrentPage::TermsOfService/>
                        }
                    />
                    <Route path="/shop/cart" view=move || view! {
                            <GlobalPage current_page=CurrentPage::ShoppingCart/>
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CurrentPage {
    HomePage,
    PetShop,
    FoodShop,
    About,
    PrivacyPolicy,
    TermsOfService,
    ShoppingCart,
}

#[component]
pub fn GlobalPage(current_page: CurrentPage) -> impl IntoView {
    view! {
        <NavBar selected=current_page/>
            <Pager page=match current_page {
                CurrentPage::HomePage => { || view!{ <crate::app::HomePage/> } }
                CurrentPage::PetShop => { || view!{ <crate::app::PetShop/> } }
                CurrentPage::FoodShop => { || view!{ <crate::app::FoodShop/> } }
                CurrentPage::About => { || view!{ <crate::app::About/> } }
                CurrentPage::PrivacyPolicy => { || view!{ <crate::app::PrivacyPolicy/> } }
                CurrentPage::TermsOfService => { || view!{ <crate::app::TermsOfService/> } }
                CurrentPage::ShoppingCart => { || view! { <crate::app::ShoppingCart/> } }
                _ => { || view!{ <crate::app::Blank/> } }
            }/>
        <FooterBar/>
    }
}

#[component]
pub fn Blank() -> impl IntoView {
    view! {}
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
    view! {
        <div>
            "Shopping cart"
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
pub fn NavBar(selected: CurrentPage) -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="logo-container">
                <a href="/">
                    <img src="/main_logo.svg" alt="Farmtasker Logo"/>
                </a>
            </div>
            <h4 class="title-text">"Marketplace for farmers & pet food manufacturers"</h4>
            <ul class="nav_buttons">
                <li>
                    <a class:current=move || {
                        matches!(selected, CurrentPage::HomePage)
                    }
                        href="/" id="button_left">"Home"</a>
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected, CurrentPage::FoodShop)
                    }
                        href="/shop/food" id="button_middle">"Food Shop"</a>
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected, CurrentPage::PetShop)
                    }
                        href="/shop/pet" id="button_middle">"Pet Shop"</a>
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected, CurrentPage::About)
                    }
                        href="/about" id="button_right">"About Us"</a>
                </li>
                <li>
                    <a
                    class:current=move || {
                        matches!(selected, CurrentPage::ShoppingCart)
                    }
                        href="/shop/cart" id="button_right">"🛒"</a>
                </li>
            </ul>
        </nav>
    }
}

use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
                    <Route path="/" view=|| view! { <GlobalPage current_page=CurrentPage::HomePage/> }/>
                    <Route path="/shop/pet" view=|| view! { <GlobalPage current_page=CurrentPage::PetShop/> }/>
                    <Route path="/shop/food" view=|| view! { <GlobalPage current_page=CurrentPage::FoodShop/> }/>
                    <Route path="/about" view=|| view! { <GlobalPage current_page=CurrentPage::About/> }/>
                    <Route path="/privacy" view=|| view! { <GlobalPage current_page=CurrentPage::PrivacyPolicy/> }/>
                    <Route path="/terms" view=|| view! { <GlobalPage current_page=CurrentPage::TermsOfService/> }/>
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
            }/>
        <FooterBar/>
    }
}

#[component]
pub fn Pager<F, IV>(page: F) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div class="pager-bg">
            <div class="pager">
                <div class="pager-content">{page()}</div>
            </div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let mut vec = Vec::new();
    for _ in 0..320 {
        vec.push(
            view! {
                <a id="shop_selector" class:red=move|| {count() % 2 == 1} href="/shop/pet">"Pet Shop"</a>
                <a id="shop_selector" class:red=move|| {count() % 2 == 1} href="/shop/food">"Food Shop"</a>
            }
        )
    }

    view! {
        {vec}
        <a id="shop_selector" class:red=move|| {count() % 2 == 1} href="/shop/pet">"Pet Shop"</a>
        <a id="shop_selector" class:red=move|| {count() % 2 == 1} href="/shop/food">"Food Shop"</a>
        <button id="test" on:click=on_click>"Click Me: " {count}</button>
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
                <a href="mailto:info@farmtasker.au">"info@farmtasker.au"</a>
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
                <a href="mailto:info@farmtasker.au">"info@farmtasker.au"</a>
                "."
            </p>
        </div>
    }
}

#[component]
pub fn FooterBar() -> impl IntoView {
    view! {
        <footer class="footerbar">
            <div class="footer-content">
                <div class="footer-section">
                    <p>"© 2024 Farmtasker. All rights reserved."</p>
                </div>
                <div class="footer-section">
                    <p>
                        "Contact us: "
                        <a href="mailto:info@farmtasker.au">"info@farmtasker.au"</a>
                    </p>
                </div>
                <div class="footer-section">
                    <p>
                        <a href="/privacy">"Privacy Policy"</a> |
                        <a href="/terms">"Terms of Service"</a>
                    </p>
                </div>
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
            <h4 class="title-text">"Farmtasker Shop"</h4>
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
                        matches!(selected, CurrentPage::PetShop)
                    }
                        href="/shop/pet" id="button_middle">"Pet Shop"</a>
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
                        matches!(selected, CurrentPage::About)
                    }
                        href="/about" id="button_right">"About Us"</a>
                </li>
            </ul>
        </nav>
    }
}

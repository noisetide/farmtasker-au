use super::CurrentPage;
use crate::*;
use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let selected = expect_context::<ReadSignal<CurrentPage>>();

    let (is_navbar_hidden, set_is_navbar_hidden) = create_signal(true);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);

    view! {
        <nav class="navbar">
            <div class="banner-bg">
                <div class="logo-container">
                    <a href="/" class="main_logo">
                        <img src="/navbar/shapka/webp/main_logo.webp" alt="Farmtasker Logo"/>
                    </a>
                    <button class="navbar-menu-button"
                        on:click=move |_| {
                            set_is_navbar_hidden.update(|n| *n = !*n);
                        }
                    >
                        <img src="/navbar/shapka/webp/menu_bars_tag.webp" alt="Menu"/>
                    </button>
                    <img src="/navbar/shapka/webp/farm_products_marketplace_tag.webp" class="navbar-welcome" alt="Welcome to farm products marketplace!"/>
                    <a href="/shop/cart" class="shopping-cart-button"
                        class:current=move || {
                            matches!(selected.get(), CurrentPage::ShoppingCart)
                        }
                    >
                        <img src="/navbar/shapka/webp/shopping_cart_tag.webp"/>
                        <div class="shopping-cart-counter">
                            {move || match shopping_cart.get().0.values().map(|&v| v as usize).sum() {
                                0 => "0".to_string(),
                                x => x.to_string(),
                            }}
                        </div>
                    </a>
                </div>
            </div>
            <ul class="nav_buttons"
                class:is-navbar-hidden=move || is_navbar_hidden()
                class:is-navbar-hidden-opposite=move || !is_navbar_hidden()
                >
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::HomePage)}
                        href="/" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Home"
                        />
                        <span class="overlay-text">Home</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::FarmFood)}
                        href="/shop/food" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Food Shop"
                        />
                        <span class="overlay-text">Farm Food</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::PetFood)}
                        href="/shop/pet" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Farm Pet Food"
                        />
                        <span class="overlay-text">Pet Food</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::ReadyToEat)}
                        href="/shop/ready-to-eat" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Ready To Eat"
                        />
                        <span class="overlay-text">Ready To Eat</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::VideoBlogs)}
                        href="/blog/culinary-adventure" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Video Blogs"
                        />
                        <span class="overlay-text">Video Blogs</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::Delivery)}
                        href="/delivery" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Delivery"
                        />
                        <span class="overlay-text">Delivery</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::About)}
                        href="/about" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="About Us"
                        />
                        <span class="overlay-text">About Us</span>
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

#[component]
pub fn FooterBar() -> impl IntoView {
    view! {
        <footer class="footerbar">
            <div class="footer-content">
                // <div class="footer-section">
                //     <p>
                //         <div>"© 2024 FARMTASKER PTY LTD. All rights reserved."</div>
                //         "Code licensed under the Lesser General Public Licence (LGPL-2.1). See "<a href="https://github.com/rotteegher/farmtasker-au" target="_blank">"source code"</a>" for details."
                //     </p>

                //     <p>
                //         "This website is licensed under the "
                //         <a href="https://www.gnu.org/licenses/lgpl-2.1.html" target="_blank">"GNU Lesser General Public License v2.1"</a>.
                //     </p>
                // </div>
                <div class="footer-section">
                    <p>
                        <div>"© 2025 FARMTASKER PTY LTD. All rights reserved."</div>
                        "Contact us: "
                        <a href="mailto:farmtasker@gmail.com">"farmtasker@gmail.com"</a>
                        " or"
                        <a href="mailto:info@farmtasker.au">" info@farmtasker.au"</a>
                        <div>" +61484753577"</div>
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

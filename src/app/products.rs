use super::AppStateDataRes;
use crate::*;
use leptos::*;
use leptos_use::*;
use log::*;

// // REIMPLEMENTED into Cfg!!!
// #[component]
// pub fn DbProductItem(product: DbProduct) -> impl IntoView {
//     let (product, _) = create_signal(product);
//     provide_context(product);

//     let shopping_cart = expect_context::<Signal<ShoppingCart>>();
//     provide_context(shopping_cart);
//     let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
//     provide_context(set_shopping_cart);

//     view! {
//         <div class="product-item-container">
//             <a href=move || {
//                 let product_name = product.get().name.to_lowercase().replace(" ", "-");
//                 format!("/shop/products/:{:#}", product_name)
//             }>
//                 <Show
//                     when=move || {product.get().images.is_some_and(|x| !x.is_empty())}
//                     fallback=move || {view!{
//                         <div class="product-item-empty">
//                         </div>
//                     }}
//                 >
//                     <img class="product-item-image" src={product.get().images.unwrap().first()}/>
//                 </Show>
//                 <div class="product-info">
//                     <strong class="product-item-name">
//                         {product.get().name}
//                     </strong>
//                 </div>
//             </a>
//             <button class="product-item-addtocart-button" on:click=move |_| {
//                 set_shopping_cart.update(|s| {
//                     s.add_single_product(&product.get().id, 20);
//                 });
//             }>
//             "Add To Cart $"{product.get().default_price.unwrap().unit_amount.unwrap() / 100}
//             </button>
//         </div>
//     }
// }

#[component]
pub fn CfgProductItem(product: CfgProduct) -> impl IntoView {
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
                    <img class="product-item-image" src=move || {
                        let image_path = product
                            .get()
                            .local_images
                            .as_ref()
                            .expect("No local images for CfgProduct!")
                            .iter()
                            .find(|image_path| image_path.file_name().map_or(false, |name| name == "thumbnail.webp"))
                            .map(|image_pathbuf| image_pathbuf.to_string_lossy().to_string()) // Convert to owned String
                            .unwrap_or_else(|| {
                                error!(
                                    "Couldn't find thumbnail.webp for product! {}",
                                    product.get().item_number.expect("No item_number of CfgProduct!")
                                );
                                "no_image_for_cfgproduct.webp".to_string() // Return owned String
                            });
                        image_path
                    }/>
                </Show>
                <div class="product-info">
                    <strong class="product-item-name">
                        {product.get().name}
                    </strong>
                </div>
            </a>
            <button class="product-item-addtocart-button" on:click=move |_| {
                set_shopping_cart.update(|s| {
                    s.add_single_product(&product.get().stripe_id, 20);
                });
            }>
            "Add To Cart $"{product.get().price.unwrap().unit_amount.unwrap() / 100}
            </button>
        </div>
    }
}

#[component]
pub fn CfgProductItemsList(items_category: String) -> impl IntoView {
    let app_state = expect_context::<AppStateDataRes>();
    provide_context(app_state);
    let (items_category, set_items_category) = create_signal(items_category);
    provide_context(items_category);

    view! {
        <Suspense fallback=move || view! {"Loading data..."}>
            {
                move || match app_state.get() {
                    None => view! { <p>"Loading..."</p>}.into_view(),
                    Some(app_state) => {
                        let products_config: CfgProducts = app_state.clone()
                            .expect("Resource AppState is not here on 'get()")
                            .products_config.expect("Resource StripeData is not here on 'get()'");
                        let items_category = expect_context::<ReadSignal<String>>();
                        provide_context(items_category);


                        view! {
                            <ul class="product-list-ul">
                                {
                                    products_config.0.into_iter()
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
                                                <CfgProductItem product=product/>
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                }
                            </ul>
                        }.into_view()
                    }
                }

            }
        </Suspense>
    }
}

#[component]
pub fn CfgProductItemDetailsPage(product_name: String) -> impl IntoView {
    // let stripe_data = expect_context::<StripeDataRes>();
    let app_state = expect_context::<AppStateDataRes>();
    provide_context(app_state);

    let (product_name, _) = create_signal(product_name);
    provide_context(product_name);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    view! {
        <Suspense fallback=move || view! {"loading data"}>
            {move || match app_state.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(app_state) => {
                    // let stripe_data: StripeData = stripe_data.expect("Resource StripeData is not here on 'get()'");
                    let products_config: CfgProducts = app_state.clone()
                        .expect("Resource AppState is not here on 'get()")
                        .products_config.expect("Resource StripeData is not here on 'get()'");

                    let product_name = expect_context::<ReadSignal<String>>();
                    provide_context(product_name);


                    match products_config.0.into_iter()
                    .find(|product| {
                        let cmp1 = product.name.to_lowercase().replace(" ", "-");
                        let cmp2 = &product_name.get()[1..];

                        cmp1 == cmp2
                    }) {
                        Some(product) => {
                            view!{
                                <CfgProductItemDetailsContent product=product/>
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
pub fn ImageListDisplay(images: Vec<std::path::PathBuf>, class: String) -> impl IntoView {
    let (img_class, _) = create_signal(class);
    provide_context(img_class);
    let (images_signal, _) = create_signal(images.clone());
    provide_context(images_signal);

    let (current_index, set_current_index) = create_signal(0usize);
    let current_image = move || {
        images_signal
            .get()
            .get(current_index.get())
            .cloned()
            .unwrap_or_default()
    };

    let (img_opacity, set_img_opacity) = create_signal(1.0);

    let change_image_with_fade = move |new_index: usize| {
        set_img_opacity.set(0.0); // fade out
        set_timeout(
            move || {
                set_current_index.set(new_index); // switch image after fade out
                set_img_opacity.set(1.0); // fade in
            },
            std::time::Duration::from_millis(300),
        );
    };

    view! {
        <div class="image_display">
            <Show
                when=move || {!images_signal.get().is_empty()} // when not empty images show
                fallback=move || {view!{
                    <div class="product-item-empty">
                        "Sorry, no images due to error. Contact support."
                    </div>
                }}
            >
                <img
                    class={img_class.get()}
                    src={move || current_image().display().to_string()}
                    style={move || format!("opacity: {}; transition: 0.1s ease-in-out;", img_opacity.get())}
                />
            </Show>
            <button class=move || {format!("{}-button-left", img_class.get())} on:click=move |_| {
                let len = images_signal.get().len();
                if len > 0 {
                    let new_index = if current_index.get() == 0 {
                        len - 1
                    } else {
                        current_index.get() - 1
                    };
                    // leptos::logging::log!("len: {}, i: {}, p: {}", len, new_index, current_image().display());
                    set_current_index.set(new_index);
                } else {
                    panic!("Images Vec<std::path::PathBuf> is empty")
                }
            }>
                "<-"
            </button>
            <button class=move || {format!("{}-button-right", img_class.get())} on:click=move |_| {
                let len = images_signal.get().len();
                if len > 0 {
                    let new_index = (current_index.get() + 1) % len;
                    // leptos::logging::log!("len: {}, i: {}, p: {}", len, new_index, current_image().display());
                    set_current_index.set(new_index);
                } else {
                    panic!("Images Vec<std::path::PathBuf> is empty")
                }
            }>
                "->"
            </button>
        </div>
    }
}

#[component]
pub fn CfgProductItemDetailsContent(product: CfgProduct) -> impl IntoView {
    let (product, _) = create_signal(product);
    provide_context(product);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    view! {
        <div class="product-item-container">
            <Show
                when=move || {product.get().local_images.is_some_and(|x| !x.is_empty())} // when not empty images show
                fallback=move || {view!{
                    <div class="product-item-empty">
                        "Sorry, no image. Contact support."
                    </div>
                }}
            >
                // <img class="product-item-image" src={product.get().local_images.unwrap().first().unwrap_or(&std::path::PathBuf::from("no_local_image_found_error")).to_owned().display().to_string()}/>
                <ImageListDisplay images={product.get().local_images.expect("None local_images")} class="product-item-image".to_string() />
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
                    s.add_single_product(&product.get().stripe_id, 20);
                });
            }>
            "Add To Cart $"{product.get().price.unwrap().unit_amount.unwrap() / 100}
            </button>
        </div>
    }
}

#[component]
pub fn CfgProductItemShoppingCartCounter(product: CfgProduct) -> impl IntoView {
    let (product, _) = create_signal(product);
    let app_state = expect_context::<AppStateDataRes>();
    provide_context(app_state);
    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);

    view! {
        <Show
            when=move || {
                shopping_cart
                    .get()
                    .0
                    .keys()
                    .find(|stripe_id| product.get().stripe_id == stripe_id.to_owned().to_owned()).unwrap().clone()
                     == app_state.get()
                        .expect("No AppState!")
                        .expect("No AppState!")
                        .products_config
                        .expect("No products_config in AppState!")
                        .0
                        .iter()
                        .find(|appstate_cfg_product| appstate_cfg_product.stripe_id == product.get().stripe_id).unwrap().stripe_id
            }
            fallback=move || {view!{
                <div>
                </div>
            }}
        >
            <div class="product-item-shopping-cart-counter" class=product.get().stripe_id>

            </div>
        </Show>
    }
}
// // REIMPLEMENTED into Cfg!!!
// #[component]
// pub fn DbProductItemsList(items_category: String) -> impl IntoView {
//     let app_state = expect_context::<AppStateDataRes>();
//     provide_context(app_state);
//     let (items_category, set_items_category) = create_signal(items_category);
//     provide_context(items_category);

//     view! {
//         <Suspense fallback=move || view! {"Loading data..."}>
//             {
//                 move || match app_state.get() {
//                     None => view! { <p>"Loading..."</p> }.into_view(),
//                     Some(app_state) => {
//                         let stripe_data: StripeData = app_state.clone()
//                             .expect("Resource AppState is not here on 'get()")
//                             .stripe_data.expect("Resource StripeData is not here on 'get()'");

//                         let products_config: CfgProducts = app_state.clone()
//                             .expect("Resource AppState is not here on 'get()")
//                             .products_config.expect("Resource StripeData is not here on 'get()'");
//                         let items_category = expect_context::<ReadSignal<String>>();
//                         provide_context(items_category);

//                         view! {
//                             <ul class="product-list-ul">
//                                 {
//                                     stripe_data.products.into_iter()
//                                     .filter(|product| {
//                                         product.metadata
//                                             .as_ref()
//                                             .and_then(|metadata| metadata.get("category"))
//                                             .map(|category| category == &items_category.get())
//                                             .unwrap_or(false)
//                                     })
//                                     .map(|product| {
//                                         view! {
//                                             <li class="product-list-item">
//                                                 <DbProductItem product=product/>
//                                             </li>
//                                         }
//                                     })
//                                     .collect::<Vec<_>>()
//                                 }
//                             </ul>
//                         }.into_view()
//                     }
//                 }
//             }
//         </Suspense>
//     }
// }


// // REIMPLEMENTED into Cfg!!!
// #[component]
// pub fn DbProductItemDetailsPage(product_name: String) -> impl IntoView {
//     let stripe_data = expect_context::<StripeDataRes>();
//     let (product_name, _) = create_signal(product_name);
//     provide_context(product_name);
//
//     let shopping_cart = expect_context::<Signal<ShoppingCart>>();
//     provide_context(shopping_cart);
//     let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
//     provide_context(set_shopping_cart);
//
//     view! {
//         <Suspense fallback=move || view! {"loading data"}>
//             {move || match stripe_data.get() {
//                 None => view! { <p>"Loading..."</p> }.into_view(),
//                 Some(stripe_data) => {
//                     let stripe_data: StripeData = stripe_data.expect("Resource StripeData is not here on 'get()'");
//                     let product_name = expect_context::<ReadSignal<String>>();
//                     provide_context(product_name);
//
//                     match stripe_data.products.into_iter()
//                     .find(|product| {
//                         let cmp1 = product.name.to_lowercase().replace(" ", "-");
//                         let cmp2 = &product_name.get()[1..];
//
//                         cmp1 == cmp2
//                     }) {
//                         Some(product) => {
//                             view!{
//                                 <DbProductItemDetails product=product/>
//                             }.into_view()
//                         },
//                         None => view!{
//                             <div>"NO PRODUCT WITH SUCH NAME"</div>
//                         }.into_view(),
//                     }
//                 }
//             }}
//         </Suspense>
//     }
// }

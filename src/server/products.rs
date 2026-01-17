use crate::{CfgProduct, CfgProducts, StripeData};
use leptos::ServerFnError;
use log::*;

#[leptos::server(
    name = RefreshLocalProductInfo,
    endpoint = "refresh_local_products_info", // WORKING BUT TODO IMPLEMENT AUTHENTIFICATION
)]
pub async fn refresh_local_product_info(rewrite: bool) -> Result<String, leptos::ServerFnError> {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;
    use stripe::*;

    tracing::info!("");
    tracing::info!("Refreshing Local CfgProducts...");

    // Retrieve the LEPTOS_SITE_ROOT environment variable for path of the data file
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "site".to_string());
    let assets_dir = std::env::var("LEPTOS_ASSETS_DIR").unwrap_or_else(|_| "public".to_string());

    // Config file paths
    let products_config_file_path = Path::new(&site_root).join("products_config.json");
    let products_config_public_file_path = Path::new(&assets_dir).join("products_config.json");

    if products_config_file_path.exists() {
        if rewrite {
            // If the file exists delete it, then recreate new one by serializing StripeData::new_fetch()
            let stripe_data = StripeData::new_fetch().await?;
            let stripe_products_config = StripeData::derive_products_config(stripe_data);

            // MAKE SCAN OF ASSET IMAGES IN DIR
            // Attach them to the local_images in CfgProduct
            let updated_stripe_products_config: CfgProducts =
                add_images_to_products_config(stripe_products_config.clone()).await?;

            write_products_config(updated_stripe_products_config, true).await
        } else {
            // Refresh file with new products from stripe api
            let stripe_data = StripeData::new_fetch().await?;
            let stripe_products_config: CfgProducts =
                StripeData::derive_products_config(stripe_data);

            let local_products_config: CfgProducts = fetch_local_product_info().await?;

            let mut h: Vec<CfgProduct> = Vec::new();
            // Add all local products to the vector
            for p in &local_products_config.0 {
                h.push(p.clone()); // Push existing products
            }

            // Add missing Stripe products to the vector
            for s in &stripe_products_config.0 {
                if !local_products_config
                    .0
                    .iter()
                    .any(|p| p.stripe_id == s.stripe_id)
                {
                    h.push(s.clone()); // Push only missing Stripe products
                }
            }

            // Create new products config by adding missing products from stripedata
            // to existing config if the local config is missing the products by id
            // MAKE SCAN OF ASSET IMAGES IN DIR
            // Attach them to the local_images in CfgProduct
            let updated_products_config: CfgProducts =
                add_images_to_products_config(CfgProducts(h)).await?;

            // Write serialized local data updated with new products from stripe api
            write_products_config(updated_products_config, true).await
        }
    } else {
        // If the file doesn't exist, just create new one by serializing StripeData::new_fetch()
        let stripe_data = StripeData::new_fetch().await?;
        let stripe_products_config: CfgProducts = StripeData::derive_products_config(stripe_data);

        // MAKE SCAN OF ASSET IMAGES IN DIR
        // Attach them to the local_images in CfgProduct
        let updated_stripe_products_config: CfgProducts =
            add_images_to_products_config(stripe_products_config.clone()).await?;

        write_products_config(updated_stripe_products_config.clone(), true).await
    }
}

/// Fetches the Product Info from local automatically deserialized json file
/// If file doesn't exist it serializes a new file from products data inside StripeData
/// Returns Vec of Products parameters like name and price and their images
#[leptos::server(
    name = FetchLocalProductInfo,
    endpoint = "fetch_products_config"
)]
pub async fn fetch_local_product_info() -> Result<CfgProducts, leptos::ServerFnError> {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;
    use stripe::*;

    info!("");
    info!("Fetching Local CfgProducts...");

    // Retrieve the LEPTOS_SITE_ROOT environment variable for path of the data file
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "site".to_string());
    let assets_dir = std::env::var("LEPTOS_ASSETS_DIR").unwrap_or_else(|_| "public".to_string());

    let products_config_file_path = Path::new(&site_root).join("products_config.json");
    let products_config_public_file_path = Path::new(&assets_dir).join("products_config.json");

    let final_products_config: CfgProducts = if products_config_file_path.exists() {
        // if file exists then just read it into CfgProducts from leptos site-root only and return fetched data
        let products_config_file_contents = std::fs::read_to_string(products_config_file_path)?;
        let products_config: CfgProducts = serde_json::from_str(&products_config_file_contents)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        for p in &products_config.0 {
            tracing::info!(
                "CONFIG: #{:?} {:?} - #{:?} AUD ",
                p.item_number.unwrap_or(-1),
                p.name,
                p.price.clone().unwrap().unit_amount.unwrap() as f64 / 100.0
            );
        }

        products_config
    } else {
        // If the file doesn't exist, create new one by serializing StripeData::new_fetch()
        let stripe_data = StripeData::new_fetch().await?;
        let stripe_products_config: CfgProducts = StripeData::derive_products_config(stripe_data);

        write_products_config(stripe_products_config.clone(), true).await?;

        stripe_products_config
    };

    tracing::info!("CONFIG TOTAL: {:?}", final_products_config.clone().0.len());

    Ok(final_products_config)
}

/// Adds images to CfgProducts from assets
#[leptos::server(
    name = AddImagesToProductsConfig
)]
async fn add_images_to_products_config(
    products_config: CfgProducts,
) -> Result<CfgProducts, ServerFnError> {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;
    use std::path::PathBuf;

    info!("Adding images to CfgProducts...");

    // Retrieve the LEPTOS_SITE_ROOT environment variable for path of the data file
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "site".to_string());

    let products_assets_dir = Path::new(&site_root);

    let mut updated_products_config = products_config.clone();

    for mut product in &mut updated_products_config.0 {
        // assert_eq!(product.images.is_some(), true);

        if let Some(item_number) = product.item_number {
            // Build the path for the product's assets directory
            let product_images_dir_path = format!(
                "{}/products_assets/{}/ready_assets/",
                site_root, item_number
            );
            let product_images_dir_path = Path::new(&product_images_dir_path);

            if product_images_dir_path.exists() {
                info!(
                    "FOUND LOCAL IMAGES DIR OF PRODUCT {:?}: {:?}",
                    item_number,
                    product_images_dir_path.display()
                );

                let webp_files: Vec<PathBuf> = match std::fs::read_dir(product_images_dir_path) {
                    Ok(entries) => entries
                        .filter_map(|entry| entry.ok()) // Filter out errors
                        .map(|entry| entry.path()) // Get the path
                        .filter(|path| {
                            path.is_file() && path.extension().map_or(false, |ext| ext == "webp")
                        })
                        .collect(),
                    Err(_) => vec![], // Return empty Vec<PathBuf> if read_dir fails
                };
                let webp_files_local: Vec<PathBuf> = webp_files
                    .into_iter()
                    .filter_map(|path| {
                        path.strip_prefix(site_root.clone())
                            .ok()
                            .map(|p| p.to_path_buf())
                    })
                    .map(|path| PathBuf::from(format!("/{}", path.display()))) // a trick to convert this path to absolute so css element <img src="path here"> displays it correctly from LEPTOS_SITE_ROOT dir
                    .collect();

                // Set images
                product.local_images = Some(webp_files_local);
                // info!(
                //     "LOCAL WEBP FILES for PRODUCT {:?}: {:#?}",
                //     item_number, webp_files_local
                // );
            } else {
                error!(
                    "DID NOT FIND PRODUCT IMAGES DIR OF PRODUCT {:?}: {:?}",
                    item_number,
                    product_images_dir_path.display()
                );
            }
        }
    }

    for product in &updated_products_config.0 {
        info!(
            "FOUND LOCAL IMAGES OF PRODUCT {:?}: {:#?}",
            product.item_number.unwrap(),
            product.local_images.clone()
        );
    }

    Ok(updated_products_config)
}

/// Writes the config file of CfgProducts
#[leptos::server(
    name = WriteProductsConfig
)]
pub async fn write_products_config(
    products_config: CfgProducts,
    rewrite: bool,
) -> Result<String, leptos::ServerFnError> {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;

    // Retrieve the LEPTOS_SITE_ROOT environment variable for path of the data file
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "site".to_string());
    let assets_dir = std::env::var("LEPTOS_ASSETS_DIR").unwrap_or_else(|_| "public".to_string());

    // Config file paths
    let products_config_file_path = Path::new(&site_root).join("products_config.json");
    let products_config_public_file_path = Path::new(&assets_dir).join("products_config.json");

    std::fs::create_dir_all(&site_root); // safe measure if dir doesn't exist, create it
    std::fs::create_dir_all(&assets_dir);

    if rewrite {
        // remove existing file
        std::fs::remove_file(products_config_file_path.clone());
        std::fs::remove_file(products_config_public_file_path.clone());
        info!("Removed products config file.")
    }

    let json_data = serde_json::to_string_pretty(&products_config)?;
    std::fs::write(&products_config_file_path, json_data.clone())?; // write to target/site/
    std::fs::write(&products_config_public_file_path, json_data.clone())?; // write to public/
    info!(
        "Written products config file with synced data at: {}",
        products_config_file_path.display()
    );
    info!(
        "Written products config file with synced data at: {}",
        products_config_public_file_path.display()
    );
    Ok("Ok".into())
}

use crate::stripe_retypes;
use crate::stripe_retypes::DbProduct;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct ShoppingCart(pub HashMap<String /* stripe_id */, u8 /* amount */>);

impl ShoppingCart {
    pub fn add_single_product(&mut self, product_id: &String, add_limit: u8) {
        // If the product is already in the cart
        if let Some(quantity) = self.0.get_mut(product_id) {
            // Ensure the quantity doesn't exceed 20
            if *quantity < add_limit {
                *quantity += 1;
            }
        } else {
            // If the product is not in the cart, add it with a quantity of 1
            self.0.insert(product_id.clone(), 1);
        }
    }
    pub fn remove_single_product(&mut self, product_id: &String) {
        // If the product is in the cart, adjust its quantity
        if let Some(quantity) = self.0.get_mut(product_id) {
            if *quantity > 1 {
                *quantity -= 1; // Decrease quantity by 1
            } else {
                self.0.remove(&product_id.clone()); // If quantity is 1, remove the product
            }
        }
    }
    pub fn total_quantity(self) -> u64 {
        self.0.values().map(|&v| v as u64).sum()
    }

    pub fn calculate_total_price(&self, stripe_data: &[DbProduct]) -> i64 {
        let mut total_price: i64 = 0;

        // Iterate over the shopping cart (product_id, quantity)
        for (product_id, &quantity) in &self.0 {
            // Find the corresponding product in stripe_data
            if let Some(product) = stripe_data.iter().find(|p| p.id == *product_id) {
                // Check if the product has a default price and if it's active
                if let Some(price) = &product.default_price {
                    if price.active {
                        // Get the unit_amount from the price, default to 0 if it's not present
                        if let Some(unit_amount) = price.unit_amount {
                            // Multiply the unit price by the quantity and add it to the total
                            total_price += unit_amount * quantity as i64;
                        }
                    }
                }
            }
        }

        total_price
    }

    pub fn delete_product(&mut self, product_id: String) {
        self.0.remove(&product_id);
    }
}

impl From<Vec<stripe_retypes::DbCheckoutSessionItem>> for ShoppingCart {
    fn from(value: Vec<stripe_retypes::DbCheckoutSessionItem>) -> Self {
        let mut cart = ShoppingCart::default();
        for item in value {
            cart.0.insert(
                item.id.to_string(),
                item.quantity.unwrap_or_default().try_into().unwrap(),
            );
        }
        cart
    }
}

impl Default for ShoppingCart {
    fn default() -> Self {
        ShoppingCart(HashMap::<String, u8>::new())
    }
}

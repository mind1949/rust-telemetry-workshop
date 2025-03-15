//! # Exercise
//!
//! We'll ignore the consumer side for now and focus on the instrumentation side.  
//! We'll redo the previous exercise using the `tracing` crate in order to get familiar with the
//! basics.
mod subscriber;

pub use subscriber::init_test_subscriber;
pub use tracing;

/// Given a list of order numbers, compute the total price.
///
/// # Exercise
///
/// Wrap `get_total` and `get_order_details`, our two units of work, in a `tracing::Span`.
/// We don't care about capturing the outcome of each unit of work (for now).
///
/// Refer to the test files for the expected output format.
#[tracing::instrument("process total price", skip_all)]
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    // Tip: use `tracing::info_span!` to create a new span.
    // You'll have to learn about the *RAII guard* pattern!
    let mut total = 0;
    for order_number in order_numbers {
        let order = get_order_details(*order_number)?;
        total += order.price;
    }
    Ok(total)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
#[tracing::instrument("retrieve order", skip_all)]
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    if order_number % 4 == 0 {
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}

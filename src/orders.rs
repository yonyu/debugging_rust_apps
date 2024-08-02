use crate::structs:: {
	Context,
	Customer,
	Order,
	OrderLineItem,
	Product,
	Store,
};

pub fn simulate_an_order() -> Context
{
	let mut context: Context = Context::new();

	// Defines the primary store 
	let primary_store_id: i32 = 0i32;
	let mut primary_store: Store = Store
	{
		id: primary_store_id,
		orders: Vec::with_capacity(32)
	};

	let product_id: i32 = 0i32;
	let drip_brew_coffee_product: Product = Product
	{
		id: product_id,
		label: String::from("Drip Brew Coffee"),
		price: 3.5f32
	};

	let customer_id: i32 = 0i32;
	let example_customer: Customer = Customer
	{
		id: customer_id,
		first_name: String::from("Example"),
		last_name: String::from("Customer"),
		email: String::from("example@pluralsight.com")
	};

	let order_number: i32 = 0i32;
	let mut order: Order = Order
	{
		id: order_number,
		customer_id: example_customer.id,
		amount_total: 0.0,
		line_item_start: context.next_line_item,
		created_date: String::from("2023-01-28")
	};

	primary_store.orders.push(order.id);

	let order_line_item_id: i32 = context.next_line_item;

	// Add Drip Brew Coffee to existing order
	add_line_item(
		&mut context,
		&mut order, 
		order_line_item_id,
		&drip_brew_coffee_product);

	context.stores.push(primary_store);
	context.customers.push(example_customer);
	context.orders.push(order);
	context.products.push(drip_brew_coffee_product);

	print!("simulate_an_order complete.\n");

	return context;
}

fn add_line_item(
	context: &mut Context,
	order: &mut Order, 
	order_line_item_id: i32, 
	product: &Product)
{
	let order_line_item: OrderLineItem = OrderLineItem
	{
		id: order_line_item_id,
		product_id: product.id,
		order_id: order.id,
		amount: product.price
	};

	order.amount_total += order_line_item.amount;
	context.order_line_items.push(order_line_item);
	context.next_line_item += 1;
}

use rayon::prelude::*;
pub fn calculate_orders_parallel()
{
    let contexts: Vec<Context> = (0..64).into_par_iter()
        .map(|_item| simulate_an_order())
        .collect();
	print!("contexts.len(): {}\n", contexts.len());
}

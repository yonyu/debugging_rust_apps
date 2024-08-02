pub struct Context
{
	pub data_path: String,
	pub stores: Vec<Store>,
	pub products: Vec<Product>,
	pub customers: Vec<Customer>,
	pub orders: Vec<Order>,
	pub order_line_items: Vec<OrderLineItem>,

	pub next_line_item: i32,

}

impl Context
{
	pub fn new() -> Context
	{
		Context
		{
			data_path: String::new(),
			products: Vec::new(),
			stores: Vec::new(),
			customers: Vec::new(),
			orders: Vec::new(),
			order_line_items: Vec::new(),

			next_line_item: 0i32
		}
	}

	pub fn print_order(&self, order: &Order)
	{
		let customer: &Customer = &self.customers[order.customer_id as usize];
		print!("Id: {}\nTotal: {}\nCustomer: {} {}\nCustomer ID: {}",
			order.id, 
			order.amount_total,
			customer.first_name,
			customer.last_name,
			order.customer_id);

		print!("\n\n");
		print!("Product\t\t\tAmount\n");
		print!("==============================\n");

		let mut line_item_index: usize = order.line_item_start as usize;

		loop
		{
			if line_item_index > &self.order_line_items.len() - 1 { break; }

			let line_item: &OrderLineItem = &self.order_line_items[line_item_index];

			if line_item.order_id != order.id { break; }

			let product: &Product = &self.products[line_item.product_id as usize];
			print!("{}\t{}", product.label, line_item.amount);	
			
			line_item_index += 1;
		}

		print!("\n");
	}
}

pub struct Store
{
	pub id: i32,
	pub orders: Vec<i32>

}

pub struct Customer
{
	pub id: i32,
	pub first_name: String,
	pub last_name: String,
	pub email: String,
}

pub struct Order
{
	pub id: i32,
	pub customer_id: i32, 
	pub line_item_start: i32,
	pub amount_total: f32,
	pub created_date: String,
}

pub struct OrderLineItem
{
	pub id: i32,
	pub order_id: i32,
	pub product_id: i32,
	pub amount: f32,
}

pub struct Product
{
	pub id: i32,
	pub label: String,
	pub price: f32,
}
use std::{thread, time::Duration};
use rquickjs::{prelude::{Func, Rest}, Context, Object, Runtime, Value};
use super::parser::{JSParser, LuaParser};

impl LuaParser {
	fn export_functions(&mut self) {
		let lua = self.lua.get_mut();

		lua.globals().set("clear_window_color", lua.create_function(|_, (red, green, blue, alpha)| {
		  clear_gl_window_color(red, green, blue, alpha);
		  Ok(())
	   }).unwrap()).unwrap();

		// lua.globals().set("setTimeout", lua.create_function(|_, (callback, delay_ms): (Function, u64)| {
		// 	Ok(())
		// }).unwrap()).unwrap();		
	}

	pub fn set_globals(&mut self) {
		self.export_functions();
		// todo
	}
}

impl JSParser {
	fn export_functions(&mut self) {
	   let context = self.context.borrow();
	   context.with(|ctx| {
		  // Example: Exporting a Rust function to JavaScript as "clear_window_color"
			let globals = ctx.globals();
		  globals.set(
			 "clear_window_color",
			 Func::new(|red: f32, green: f32, blue: f32, alpha: f32| {
				clear_gl_window_color(red, green, blue, alpha);
			 })
		  ).unwrap();
			

			// Create a `console` object
			let console = Object::new(ctx).unwrap();

			// Set the `log` function on the `console` object
			console.set("log", Func::new(console_log)).unwrap();
			console.set("warn", Func::new(console_log)).unwrap();
			console.set("error", Func::new(console_log)).unwrap();
		
			// Set the `console` object globally
			globals.set("console", console).unwrap();

			globals.set(
				"setTimeout",
				Func::new(move |callback: String, delay_ms: u64| {
					thread::spawn(move || {
						thread::sleep(Duration::from_millis(delay_ms));
						let rt = Runtime::new().unwrap();
						let ctx = Context::full(&rt).unwrap();
						ctx.with(|ct| {
							ct.eval::<(), _>(format!("({})()", callback)).unwrap();
						});
					});
				})
			).unwrap();
	   });
	}

	pub fn set_globals(&mut self) {
		self.context.borrow().with(|ctx| {
			ctx.globals().set("myGlobal", 42).unwrap();
		});
		self.export_functions();
		// todo
	}
}

// Define a Rust function that you want to expose to Lua
fn clear_gl_window_color(red: f32, green: f32, blue: f32, alpha: f32) {
	unsafe { gl::ClearColor(red, green, blue, alpha) };
}

fn print_value(value: Value) {
	let mut message = Vec::new();

	if let Some(string_value) = value.clone().into_string() {
	   message.push(string_value.to_string().unwrap());
	} else if let Some(num_value) = value.as_int() {
	   message.push(num_value.to_string());
	} else if value.is_array() {
	   let length = value.as_array().unwrap().len();
	   let mut array_items = Vec::new();
	   for i in 0..length {
		  let item = value.as_array().unwrap().get(i).unwrap();
		  let mut item_message = Vec::new();
		  print_value_into(item, &mut item_message);
		  array_items.push(item_message.join(", ").trim().to_string());
	   }
	   message.push(format!("[{}]", array_items.join(", ")));
	} else {
	   message.push("Unknown type".to_string());
	}

	print!("{} ", message.join(", ").trim());
}

// fn set_timeout(delay_ms: u64, callback: String) {
//	thread::spawn(move || {``
// 		let rt = Runtime::new().unwrap();
//		let ctx = Context::full(&rt).unwrap();
		
// 		ctx.with(|ct| {
// 			thread::sleep(Duration::from_millis(delay_ms));
// 			// Send the callback to the main thread
// 			ct.eval::<(), _>(format!("({})()", callback)).unwrap();
// 		})
//	});
// }

// Helper function to populate message vector
fn print_value_into(value: Value, message: &mut Vec<String>) {
	if let Some(string_value) = value.clone().into_string() {
	   message.push(string_value.to_string().unwrap());
	} else if let Some(num_value) = value.as_int() {
	   message.push(num_value.to_string());
	} else if value.is_array() {
	   let length = value.as_array().unwrap().len();
	   let mut array_items = Vec::new();
	   for i in 0..length {
		  let item = value.as_array().unwrap().get(i).unwrap();
		  let mut item_message = Vec::new();
		  print_value_into(item, &mut item_message);
		  array_items.push(item_message.join(", ").trim().to_string());
	   }
	   message.push(format!("[{}]", array_items.join(", ")));
	} else {
	   message.push("Unknown type".to_string());
	}
}

fn console_log(
	values: Rest<Value>
) {
	for value in values.0 {
		print_value(value);
	}
	println!();
}

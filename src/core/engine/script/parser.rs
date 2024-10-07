use std::{collections::HashMap, fs};

use mlua::prelude::*;
use rquickjs::{CatchResultExt, CaughtError, Context, Runtime};

pub struct LuaParser {
    pub lua: std::cell::RefCell<Lua>,
	pub loaded: HashMap<String, String>,
    pub files: Vec<String>
}

impl LuaParser {
	pub fn setup() -> Self {
		let lua = Lua::new();
		
		LuaParser {
			lua: lua.into(),
			loaded: HashMap::new(),
			files: Vec::new()
		}
	}

	pub fn init_globals(&mut self) {
		self.set_globals();
	}

	pub fn add(&mut self, file: String) {
		self.files.push(file);
	}

	pub fn load(&mut self) {
        for file in self.files.iter() {
            // Read the current content of the file
            let file_content = fs::read_to_string(file)
                .unwrap_or_else(|_| panic!("Failed to read '{}' file and couldn't parse it.", file));
            
            // Check if the file content has changed
            if let Some(old_content) = self.loaded.get(file) {
                if old_content == file_content.trim() {
                    // Content hasn't changed, skip loading
                    continue;
                }
            }

            // Load and execute the new script
            let lua = self.lua.get_mut();
            lua.load(&file_content).exec().unwrap_or_else(|err| {
				eprintln!("=> Failed to execute '{}':\nOutput: {}", file.clone(), err);
			});

            // Update the hash map with the new content
            self.loaded.insert(file.to_string(), file_content.trim().to_string());
        }
    }
}

pub struct JSParser {
    pub runtime: std::cell::RefCell<Runtime>,
    pub context: std::cell::RefCell<Context>,
    pub loaded: HashMap<String, String>,
    pub files: Vec<String>,
}

impl JSParser {
    pub fn setup() -> Self {
        let rt = Runtime::new().unwrap();
        let ctx = Context::full(&rt).unwrap();
        
        JSParser {
            runtime: std::cell::RefCell::new(rt),
            context: std::cell::RefCell::new(ctx),
            loaded: HashMap::new(),
            files: Vec::new(),
        }
    }

    pub fn init_globals(&mut self) {
		self.set_globals();
    }

    pub fn add(&mut self, file: String) {
        self.files.push(file);
    }

    pub fn load(&mut self) {
        for file in self.files.iter() {
            // Read the file content
            let file_content = fs::read_to_string(file)
                .unwrap_or_else(|_| panic!("Failed to read '{}' file and couldn't parse it.", file));
            
            // Check if the file content has changed
            if let Some(old_content) = self.loaded.get(file) {
                if old_content == file_content.trim() {
                    // Content hasn't changed, skip loading
                    continue;
                }
            }

            // Load and execute the new script in the JS context
            let context = self.context.borrow();
            context.with(|ctx| {
                if let Err(CaughtError::Error(err)) = ctx.eval::<(), _>(file_content.clone()).catch(&ctx) {
                    eprintln!("=> Failed to execute '{}':\nOutput: {}", file, err);
                }
            });

            // Update the hash map with the new content
            self.loaded.insert(file.to_string(), file_content.trim().to_string());
        }
    }
}

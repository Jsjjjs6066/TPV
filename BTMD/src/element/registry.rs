use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use crate::element::Element;



// static ELEMENT_REGISTRY: LazyLock<Mutex<HashMap<String, Box<dyn Element + Send + Sync>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
	

// pub fn register_element(name: String, element: Box<dyn Element + Send + Sync>) {
// 	let mut registry = ELEMENT_REGISTRY.lock().unwrap();
// 	registry.insert(name, element);
// }

// pub fn get_element(name: &str) -> Box<dyn Element + Send + Sync> {
// 	let registry = ELEMENT_REGISTRY.lock().unwrap();
// 	let test = registry.get(name).unwrap();
//     println!("pls1");
// 	let test2 = test;
//     println!("pls2");
// 	test2
// }

// pub fn add_alias(alias: String, target: &str) {
// 	register_element(alias, get_element(target));
// }
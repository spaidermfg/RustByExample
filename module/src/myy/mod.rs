mod inaccessible;
pub mod nested;


pub fn function() {
    println!("called my::function()");
}

fn private_function() {
    println!("callled my::private_function()");
}

pub fn indirect_access() {
    println!("called my::indirect_access(), that ->");
    private_function();
}

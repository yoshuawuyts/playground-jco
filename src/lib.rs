mod bindings;

use bindings::Guest;
use wit_bindgen::rt::string::String as WitString;

struct Component;
impl Guest for Component {
    fn scream(input: WitString) -> WitString {
        input.to_uppercase().replace(' ', "_")
    }
}

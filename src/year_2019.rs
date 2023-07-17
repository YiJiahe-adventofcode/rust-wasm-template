use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Day1 {
    input: String,
    answer: String,
}

#[wasm_bindgen]
impl Day1 {
    pub fn parse_input(&mut self, s: String) {
        self.input = s;
    }

    pub fn next(&self) {

    }

    pub fn answer(&self) -> String {
        return self.answer.to_string();
    }
}
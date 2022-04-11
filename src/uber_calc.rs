use heapless::{String, Vec};

const VALUE_STACK_DEPTH: usize = 16;
const INPUT_BUFFER_LENGHT: usize = 32;

pub struct UberCalc {
    value_stack: Vec<f64, VALUE_STACK_DEPTH>,
    input_buffer: String<INPUT_BUFFER_LENGHT>,
}

impl UberCalc {
    pub fn new() -> Self {
        UberCalc {
            value_stack: Vec::<f64, VALUE_STACK_DEPTH>::new(),
            input_buffer: String::<INPUT_BUFFER_LENGHT>::new(),
        }
    }

    pub fn push_value(&mut self, value: f64) -> &mut Self {
        self.value_stack.push(value).unwrap();
        self
    }

    pub fn add(&mut self) -> &mut Self {
        if !(self.value_stack.len() < 2) {
            let x = self.value_stack.pop().unwrap();
            let y = self.value_stack.pop().unwrap();
            let sum = x + y;
            self.value_stack.push(sum).unwrap();
        }
        self
    }

    pub fn get_value_stack(&mut self) -> &Vec<f64, VALUE_STACK_DEPTH> {
        &self.value_stack
    }

    pub fn push_input_buffer(&mut self, character: &str) -> &mut Self {
        self.input_buffer.push_str(character).unwrap();
        self
    }

    pub fn flush_input_buffer(&mut self) -> &mut Self {
        if !self.input_buffer.is_empty() {
            let val: f64 = self.input_buffer.trim().parse().unwrap();
            self.push_value(val).input_buffer.clear();
        }
        self
    }
}

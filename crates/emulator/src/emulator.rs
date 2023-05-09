use std::fmt::{Display, Formatter, Write};
use std::mem::size_of;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InputOutOfBounds { index: usize, input_count: usize },
    InvalidInputCount { supplied: usize, expected: usize },
}

pub struct Emulator {
    input_count: usize,
    component: Component,
}

impl Emulator {
    pub fn new(input_count: usize, component: Component) -> Result<Self> {
        component.check_bounds(input_count)?;
        Ok(Self {
            input_count,
            component,
        })
    }

    pub fn emulate(&self, inputs: &[bool]) -> Result<bool> {
        if self.input_count != inputs.len() {
            return Err(Error::InvalidInputCount {
                supplied: inputs.len(),
                expected: self.input_count,
            });
        }
        Ok(self.component.emulate(inputs))
    }

    pub fn emulate_all(&self) -> Result<EmulationResult> {
        assert!(
            self.input_count < size_of::<usize>(),
            "Too many inputs to emulate all possible states"
        );
        let count = 2usize.pow(self.input_count as u32);
        let mut result = EmulationResult {
            input_count: self.input_count,
            states: Vec::with_capacity(count),
        };
        let mut inputs = vec![false; self.input_count];
        for i in 0..count {
            for bit_offset in 0..self.input_count {
                let bit = (i >> bit_offset) & 1;
                inputs[self.input_count - bit_offset - 1] = bit != 0;
            }
            let state = self.emulate(&inputs)?;
            result.states.push(state);
        }
        Ok(result)
    }
}

pub struct EmulationResult {
    input_count: usize,
    states: Vec<bool>,
}

impl Display for EmulationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.input_count {
            f.write_fmt(format_args!("I{i:<2} "))?;
        }
        f.write_str("O1\n")?;
        for (i, state) in self.states.iter().enumerate() {
            for bit_offset in (0..self.input_count).rev() {
                let bit = (i >> bit_offset) & 1;
                f.write_fmt(format_args!("{bit}   "))?;
            }
            if *state {
                f.write_char('1')?;
            } else {
                f.write_char('0')?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub enum Component {
    Input { index: usize },
    Not(NotGate),
    Or(OrGate),
    And(AndGate),
    Xor(XorGate),
}

impl Component {
    fn check_bounds(&self, input_count: usize) -> Result<()> {
        match self {
            Component::Input { index } => {
                if *index >= input_count {
                    return Err(Error::InputOutOfBounds {
                        index: *index,
                        input_count,
                    });
                }
                Ok(())
            }
            Component::Not(not) => not.check_bounds(input_count),
            Component::Or(or) => or.check_bounds(input_count),
            Component::And(and) => and.check_bounds(input_count),
            Component::Xor(xor) => xor.check_bounds(input_count),
        }
    }

    fn emulate(&self, inputs: &[bool]) -> bool {
        match self {
            Component::Input { index } => inputs[*index],
            Component::Not(not) => not.emulate(inputs),
            Component::Or(or) => or.emulate(inputs),
            Component::And(and) => and.emulate(inputs),
            Component::Xor(xor) => xor.emulate(inputs),
        }
    }
}

pub fn input(index: usize) -> Component {
    Component::Input { index }
}

pub struct NotGate {
    input: Box<Component>,
}

impl NotGate {
    fn check_bounds(&self, input_count: usize) -> Result<()> {
        self.input.check_bounds(input_count)
    }

    fn emulate(&self, inputs: &[bool]) -> bool {
        !self.input.emulate(inputs)
    }
}

pub fn not(component: Component) -> Component {
    Component::Not(NotGate {
        input: Box::new(component),
    })
}

pub struct OrGate {
    inputs: Vec<Component>,
}

impl OrGate {
    fn check_bounds(&self, input_count: usize) -> Result<()> {
        for input in &self.inputs {
            input.check_bounds(input_count)?;
        }
        Ok(())
    }

    fn emulate(&self, inputs: &[bool]) -> bool {
        for input in &self.inputs {
            if input.emulate(inputs) {
                return true;
            }
        }
        false
    }
}

pub fn or(components: impl IntoIterator<Item = Component>) -> Component {
    let mut inputs = Vec::new();
    for component in components.into_iter() {
        inputs.push(component);
    }
    assert!(inputs.len() > 1, "Or gate requires at least two inputs");
    Component::Or(OrGate { inputs })
}

pub struct AndGate {
    inputs: Vec<Component>,
}

impl AndGate {
    fn check_bounds(&self, input_count: usize) -> Result<()> {
        for input in &self.inputs {
            input.check_bounds(input_count)?;
        }
        Ok(())
    }

    fn emulate(&self, inputs: &[bool]) -> bool {
        for input in &self.inputs {
            if !input.emulate(inputs) {
                return false;
            }
        }
        true
    }
}

pub fn and(components: impl IntoIterator<Item = Component>) -> Component {
    let mut inputs = Vec::new();
    for component in components.into_iter() {
        inputs.push(component);
    }
    assert!(inputs.len() > 1, "And gate requires at least two inputs");
    Component::And(AndGate { inputs })
}

pub struct XorGate {
    inputs: Vec<Component>,
}

impl XorGate {
    fn check_bounds(&self, input_count: usize) -> Result<()> {
        for input in &self.inputs {
            input.check_bounds(input_count)?;
        }
        Ok(())
    }

    fn emulate(&self, inputs: &[bool]) -> bool {
        let mut check = false;
        for input in &self.inputs {
            let result = input.emulate(inputs);
            if result && check {
                return false;
            }
            if result {
                check = true;
            }
        }
        check
    }
}

pub fn xor(components: impl IntoIterator<Item = Component>) -> Component {
    let mut inputs = Vec::new();
    for component in components.into_iter() {
        inputs.push(component);
    }
    assert!(inputs.len() > 1, "And gate requires at least two inputs");
    Component::Xor(XorGate { inputs })
}

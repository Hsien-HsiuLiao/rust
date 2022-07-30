#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add
            | CalculatorInput::Subtract
            | CalculatorInput::Multiply
            | CalculatorInput::Divide
                if stack.len() >= 2 =>
            {
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                match input {
                    CalculatorInput::Add => stack.push(operand1 + operand2),
                    CalculatorInput::Subtract => stack.push(operand1 - operand2),
                    CalculatorInput::Multiply => stack.push(operand1 * operand2),
                    CalculatorInput::Divide => stack.push(operand1 / operand2),
                    _ => (),
                }
            }
            CalculatorInput::Value(num) => stack.push(*num),
            _ => return None,
        }
    }

    (stack.len() == 1).then(|| stack[0])
}

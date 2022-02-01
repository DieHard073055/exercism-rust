#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut operation_stack = vec![];
    let mut computed = false;
    if inputs.len() == 1{
        if let Some(&CalculatorInput::Value(v)) = inputs.get(0) {
            return Some(v);
        }
    }
    for input in inputs{
        match input {
            &CalculatorInput::Value(x) => {
                operation_stack.push(x)
            },
            input => {
                computed = true;
                if operation_stack.len() < 2{
                    return  None;
                }
                match input{
                    &CalculatorInput::Add => {
                        let a = operation_stack.pop().unwrap();
                        let b = operation_stack.pop().unwrap();
                        operation_stack.push(
                            a + b
                        )
                        
                    },
                    &CalculatorInput::Subtract => {
                        let a = operation_stack.pop().unwrap();
                        let b = operation_stack.pop().unwrap();
                        operation_stack.push(
                            b - a
                        )
                        
                    },
                    &CalculatorInput::Multiply => {
                        let a = operation_stack.pop().unwrap();
                        let b = operation_stack.pop().unwrap();
                        operation_stack.push(
                            a * b
                        )
                        
                    },
                    &CalculatorInput::Divide => {
                        let a = operation_stack.pop().unwrap();
                        let b = operation_stack.pop().unwrap();
                        operation_stack.push(
                            b / a
                        )
                        
                    },
                    _ => unreachable!("unidentified input {:?}", input),
                }
            }
        };
    }
    if operation_stack.len() < 1 || ! computed {
        None
    }
    else{
        Some(operation_stack.pop().unwrap())
    }
}

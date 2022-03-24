use interpreter_composable::interpret::interpret;
use interpreter_composable::types::ByteCode;


#[cfg(test)]
mod interpreter_tests {
    use super::*;

    #[test]
    fn can_load_value_and_return() {
        use ByteCode::*;

        let result = vec![LoadVal(2), Return];

        assert_eq!(
            interpret(result).unwrap().value,
            2,
            "Should return the chosen loaded value"
        );
    }

    #[test]
    fn should_show_error_when_no_value_loaded() {
        use ByteCode::*;
        assert!(
            interpret(vec![Return]).is_err(),
            "Should return StackUnderflow error"
        );
    }
  
    #[test]
    fn can_load_and_multiply_two_values_and_return() {
        use ByteCode::*;

        let a = 2;
        let b = 4;

        let result = vec![LoadVal(a), LoadVal(b), Mul, Return];

        assert_eq!(
            interpret(result)
                .unwrap()
                .value,
            a * b,
            "Should return {} * {} = {}",
            a,
            b,
            a * b
        );
    }

    #[test]
    fn can_load_and_divide_two_values_and_return() {
        use ByteCode::*;

        let a = 4;
        let b = 2;

        let result = vec![LoadVal(a), LoadVal(b), Div, Return];

        assert_eq!(
            interpret(result)
                .unwrap()
                .value,
            a / b,
            "Should return {} / {} = {}",
            a,
            b,
            a / b
        );
    }

    #[test]
    fn can_load_and_subtract_two_values_and_return() {
        use ByteCode::*;

        let a = 4;
        let b = 2;

        let result = vec![LoadVal(a), LoadVal(b), Sub, Return];

        assert_eq!(
            interpret(result)
                .unwrap()
                .value,
            a - b,
            "Should return {} - {} = {}",
            a,
            b,
            a - b
        );
    }

    #[test]
    fn can_write_value() {
        use ByteCode::*;

        let val = 2;
        let var = 'c';

        let write_result = vec![LoadVal(val), WriteVar(var), Return];
        let interpret_write_result = interpret(write_result).unwrap();

        assert_eq!(
            interpret_write_result.variable,
            Some(var),
            "Should load value {} into variable {} -> this expression should resolve to Some({})",
            val,
            var,
            var
        );
        assert_eq!(
            interpret_write_result.value, val,
            "Should load value {} into variable {} -> this expression should resolve to {}",
            val, var, val
        );
    }

    #[test]
    fn can_perform_arithmetic_written_values() {
        use ByteCode::*;

        let arithmetic_values = vec![      
            LoadVal(1),           
            WriteVar('x'),          
            LoadVal(2),          
            WriteVar('y'),          
            ReadVar('x'),         
            LoadVal(1),           
            Add,           
            ReadVar('y'),           
            Mul,           
            LoadVal(3),           
            Sub,
            // Return the result
            Return,
        ];

        assert_eq!(
            interpret(arithmetic_values).unwrap().value,
            1,
            "Should apply various arithmetic operations using the memory on the stack and then return the result"
        );
    }
}

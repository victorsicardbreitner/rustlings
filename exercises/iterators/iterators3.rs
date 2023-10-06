// iterators3.rs
//
// This is a bigger exercise than most of the others!
// You can do it! 
// Here is your mission, should you choose to accept it:
//
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and list_of_results functions.
//
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a hint.


#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b==0 {
        Err(DivisionError::DivideByZero)  
    }
    else if a%b==0 {
        Ok(a/b)
    }
    else {
        Err(DivisionError::NotDivisible(NotDivisibleError{dividend : a, divisor : b}))
    }
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));

    let mut new_vec : Vec<i32> = vec![];
    let mut le_vecteur_contient_une_erreur : bool = false;
    let mut l_erreur_concernee_si_elle_existe = DivisionError::DivideByZero;

    for element in division_results {
        if(element.is_ok()){
            new_vec.push(element.unwrap());
        }
        else if(element.is_err()){
            le_vecteur_contient_une_erreur=true;
            l_erreur_concernee_si_elle_existe=element.unwrap_err();
        }
    }
    if(le_vecteur_contient_une_erreur){
        Err(l_erreur_concernee_si_elle_existe)
    }
    else{
        Ok(new_vec)
    }
    
   
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    let mut new_vec : Vec<Result<i32, DivisionError>> = vec![];

    for element in division_results {
        new_vec.push(element);
    }

    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}

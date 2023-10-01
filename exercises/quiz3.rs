// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written in Rust! 
// Currently the system only supports creating report cards where the student's grade is represented numerically (e.g. 1.0 -> 5.5). 
// However, the school also issues alphabetical grades (A+ -> F-) and needs to be able to print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block to support alphabetical report cards. 
// Change the Grade in the second test to "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.


use core::any::TypeId;

pub trait NumericalTrait<T> {
    fn get_grade_num(&self) -> T;
}

pub trait AlphabeticalTrait<T> {
    fn get_grade_alph(&self) -> String {
        "A+".to_string()
    }
}

pub struct ReportCard<T> where T: Clone {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,

}

impl<T : ToString + 'static + std::clone::Clone> AlphabeticalTrait<T> for ReportCard<T> {
    fn get_grade_alph(&self) -> String{
        if TypeId::of::<String>() == TypeId::of::<T>() {
            self.grade.to_string()
        } else {
           "A+".to_string()
        }
    }
}

impl<T : Clone > NumericalTrait<T> for ReportCard<T> {
    fn get_grade_num(&self) -> T{
        self.grade.clone()
    }
}


impl<T: std::fmt::Display  + 'static + std::clone::Clone> ReportCard<T> {
    pub fn print_num(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.get_grade_num())
    }

    pub fn print_alph(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.get_grade_alph())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print_num(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print_alph(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}

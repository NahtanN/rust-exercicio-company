use std::{collections::HashMap, io};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum Department {
    Engineering,
    Sales,
    HumanResources,
    Financial,
}

pub struct Company {
    pub employees: HashMap<String, Department>,
    pub department: HashMap<Department, Vec<String>>,
}

impl Company {
    pub fn new() -> Company {
        let employees: HashMap<String, Department> = HashMap::new();
        let department: HashMap<Department, Vec<String>> = HashMap::new();

        Company {
            employees,
            department,
        }
    }

    pub fn add_employee(&self) {
        println!("add_employee")

        // let mut employee_name = String::new();

        // print!("Employee name: ");

        // io::stdin()
        //     .read_line(&mut employee_name)
        //     .expect("Some erro!");

        // print!("Employee department: ");

        // for department in Department::iter() {
        //     // println!("{}")
        // }

        // io::stdin()
        //     .read_line(&mut employee_name)
        //     .expect("Some erro!");
    }

    pub fn all_in_department(&self) {
        println!("all_in_department")
    }

    pub fn all_in_company(&self) {
        println!("all_in_company")
    }
}

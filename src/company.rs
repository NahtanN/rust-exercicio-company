use std::{collections::HashMap, io};

use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

// https://github.com/rust-lang/rust/issues/22756
// Why PartialEq, Eq and Hash traits are necessary to use enum as a key on the HashMap
#[derive(Debug, EnumCountMacro, EnumIter, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Department {
    Engineering,
    Sales,
    HumanResources,
    Financial,
}

pub struct DepartmentData {
    id: u16,
    description: String,
}

impl Department {
    fn data(&self) -> DepartmentData {
        match self {
            Self::Engineering => DepartmentData {
                id: 1,
                description: "Enginnering".to_string(),
            },
            Self::Sales => DepartmentData {
                id: 2,
                description: "Sales".to_string(),
            },
            Self::Financial => DepartmentData {
                id: 3,
                description: "Financial".to_string(),
            },
            Self::HumanResources => DepartmentData {
                id: 4,
                description: "Human Resources".to_string(),
            },
        }
    }
}

pub struct Company {
    pub employees: HashMap<String, Department>,
    pub department: HashMap<Department, Vec<String>>,
}

impl Company {
    pub fn new() -> Company {
        let employees_hash: HashMap<String, Department> = HashMap::new();
        let mut department_hash: HashMap<Department, Vec<String>> = HashMap::new();

        for department in Department::iter().sorted() {
            department_hash.insert(department, Vec::new());
        }

        Company {
            employees: employees_hash,
            department: department_hash,
        }
    }

    pub fn add_employee(&mut self) {
        let mut employee_name = String::new();

        println!("Employee name: ");

        io::stdin()
            .read_line(&mut employee_name)
            .expect("Some erro!");

        println!("Employee department: ");

        let department = self.select_department();

        self.employees
            .insert(employee_name.clone(), department.clone());

        let department_hash = self.department.get_mut(&department);

        match department_hash {
            Some(hash) => hash.push(employee_name),
            None => panic!("The departament hashmap was not initialized!"),
        }
    }

    fn select_department(&self) -> Department {
        let department_count_options = Department::COUNT as u16;

        loop {
            for department in Department::iter() {
                let department_data = department.data();

                println!("{} - {}", department_data.id, department_data.description)
            }

            let mut choice = String::new();

            io::stdin().read_line(&mut choice).expect("Some error!");

            let selected_choice = choice.trim().parse::<u16>();

            match selected_choice {
                Ok(num) => match num > department_count_options || num < 1 {
                    true => println!("Please, select a valid option!"),
                    false => {
                        for option in Department::iter() {
                            let option_data = option.data();

                            if num == option_data.id {
                                return option;
                            }
                        }
                    }
                },
                Err(_) => {
                    println!("Please, select a number");
                }
            }
        }
    }

    pub fn all_in_department(&self) {
        for (k, v) in &self.department {
            println!("{} - {:?}", k.data().description, v);
        }
    }

    pub fn all_in_company(&self) {
        for key in self.employees.keys().sorted() {
            let department = &self.employees[key].data().description;

            println!("{} - {}", key, department);
        }
    }
}

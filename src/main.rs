// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company. For example, “Add Sally
// to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list
// of all people in a department or all people in the company by department,
// sorted alphabetically.

use std::collections::HashMap;
use std::io;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, EnumCountMacro, EnumIter)]
enum CompanyOptions {
    AddEmployee,
    AllInDepartment,
    AllInCompany,
    Close,
}

struct Choice {
    id: u16,
    description: String,
}

#[derive(EnumIter)]
enum Department {
    Engineering,
    Sales,
    HumanResources,
    Financial,
}

struct Company {
    employees: HashMap<String, Department>,
    department: HashMap<Department, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        let employees: HashMap<String, Department> = HashMap::new();
        let department: HashMap<Department, Vec<String>> = HashMap::new();

        Company {
            employees,
            department,
        }
    }

    fn add_employee(&self) {
        let mut employee_name = String::new();

        print!("Employee name: ");

        io::stdin()
            .read_line(&mut employee_name)
            .expect("Some erro!");

        print!("Employee department: ");

        for department in Department::iter() {
            // println!("{}")
        }

        io::stdin()
            .read_line(&mut employee_name)
            .expect("Some erro!");
    }

    fn all_in_department(&self) {}

    fn all_in_company(&self) {}
}

impl CompanyOptions {
    fn count_options() -> u16 {
        CompanyOptions::COUNT as u16
    }

    fn data(&self) -> Choice {
        match self {
            Self::AddEmployee => Choice {
                id: 1,
                description: "Add employee".to_string(),
            },
            Self::AllInDepartment => Choice {
                id: 2,
                description: "Get all people in a department".to_string(),
            },
            Self::AllInCompany => Choice {
                id: 3,
                description: "Get all people in the company".to_string(),
            },
            Self::Close => Choice {
                id: 4,
                description: "Close".to_string(),
            },
        }
    }
}

fn main() {
    let company = Company::new();

    loop {
        let option = company_options();

        match option {
            CompanyOptions::AddEmployee => company.add_employee(),
            CompanyOptions::AllInDepartment => company.all_in_department(),
            CompanyOptions::AllInCompany => company.all_in_company(),
            CompanyOptions::Close => break,
        }
    }

    println!("Finished")
}

fn company_options() -> CompanyOptions {
    let close_data = CompanyOptions::Close.data();

    let company_count_options: u16 = CompanyOptions::count_options();

    loop {
        println!("============");

        for option in CompanyOptions::iter() {
            let option_data = option.data();

            println!("{} - {}", option_data.id, option_data.description);
        }

        println!("============");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Some error!");

        let selected_choice = choice.trim().parse::<u16>();

        match selected_choice {
            Ok(num) => match num != close_data.id && (num > company_count_options || num < 1) {
                true => println!("Please, select a valid option!"),
                false => {
                    for option in CompanyOptions::iter() {
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

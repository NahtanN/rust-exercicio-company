// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company. For example, “Add Sally
// to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list
// of all people in a department or all people in the company by department,
// sorted alphabetically.

mod company;
mod company_options;

use crate::company::Company;
use crate::company_options::CompanyOptions;
use std::io;
use strum::IntoEnumIterator;

fn main() {
    let mut company = Company::new();

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

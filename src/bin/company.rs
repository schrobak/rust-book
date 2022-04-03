use book::company::{parse_command, Command};
use std::collections::HashMap;
use std::io::stdin;

fn read_line() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error reading word from stdin");
    let input = input.trim();
    String::from(input)
}

pub fn main() {
    println!("Company employees management");

    let mut company_employees = HashMap::new();

    loop {
        let command = read_line();
        match parse_command(&command) {
            Command::None => println!("No command provided"),
            Command::Unknown(cmd) => println!("Unknown command {}", cmd),
            Command::AddEmployeeToDepartment {
                department,
                employee,
            } => {
                let department = company_employees.entry(department).or_insert(Vec::new());
                if !department.contains(&employee) {
                    department.push(employee);
                }
            }
            Command::ListDepartmentEmployees { department } => {
                match company_employees.get(&department) {
                    Some(employees) => {
                        println!(
                            "There are {} employees in the {} department:",
                            employees.len(),
                            department
                        );
                        let mut employees = employees.clone();
                        employees.sort();
                        println!("{}", employees.join(", "))
                    }
                    None => println!("No employees works in {}", department),
                }
            }
            Command::ListCompanyEmployees => {
                let mut employees_number = 0;
                let mut all_employees = Vec::new();

                for (_, employees) in company_employees.iter() {
                    employees_number += employees.len();
                    all_employees.append(&mut employees.clone());
                }

                all_employees.sort();

                println!("There are {} employees in the company:", employees_number);
                println!("{}", all_employees.join(", "));
            }
        }
    }
}

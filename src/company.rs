pub enum Command {
    AddEmployeeToDepartment {
        employee: String,
        department: String,
    },
    ListDepartmentEmployees {
        department: String,
    },
    ListCompanyEmployees(),
    Unknown(String),
    None,
}

pub fn parse_command(command: &str) -> Command {
    let parts: Vec<&str> = command.split_whitespace().collect();
    match parts.as_slice() {
        ["add", employee, "to", department] => Command::AddEmployeeToDepartment {
            employee: employee.to_string(),
            department: department.to_string(),
        },
        ["list", department] => Command::ListDepartmentEmployees {
            department: department.to_string(),
        },
        ["list"] => Command::ListCompanyEmployees(),
        [cmd, ..] => Command::Unknown(cmd.to_string()),
        [] => Command::None,
    }
}

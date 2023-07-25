// Using a hash map and vectors,
// create a text interface to allow a user to add employee names to a department in a company.
//     For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of:
//     1. all people in a department or
//     2. all people in the company by department,
//                                                  (sorted alphabetically)

use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum DepartmentStyle {
    Sales,
    Finance,
    Marketing,
    Operations,
    Development,
    IT,
    HR,
    UNKNOWN,
}

#[derive(Debug, Clone)]
struct Department {
    employees: Vec<String>,
}

use std::collections::HashMap;

pub fn run() {
    let mut departments: HashMap<DepartmentStyle, Department> = HashMap::new();

    println!("These are the registered departments:");
    for style in DepartmentStyle::iter() {
        let dept = Department { employees: vec![] };
        departments.insert(style, dept);
        println!("\t - {:?}", style)
    }

    loop {
        departments = handle_user_options(departments);
    }
}

fn get_department_style_from_string(department_name: String) -> DepartmentStyle {
    match department_name.trim() {
        "sales" => DepartmentStyle::Sales,
        "finance" => DepartmentStyle::Finance,
        "marketing" => DepartmentStyle::Marketing,
        "operations" => DepartmentStyle::Operations,
        "development" => DepartmentStyle::Development,
        "hr" => DepartmentStyle::HR,
        "it" => DepartmentStyle::IT,
        _ => DepartmentStyle::UNKNOWN,
    }
}

fn handle_user_options(
    mut departments: HashMap<DepartmentStyle, Department>,
) -> HashMap<DepartmentStyle, Department> {
    println!("1 to add, 2 to list for a department, 3 to list all departments");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    match user_input.as_str().trim() {
        "1" => departments = add_employee_process(departments),
        "2" => {
            println!("What department's employees do you want to see?");
            let mut department_name_str = String::new();
            io::stdin()
                .read_line(&mut department_name_str)
                .expect("Failed to read line");
            let department_style = get_department_style_from_string(department_name_str);
            show_employees_by_department(&departments, department_style);
        }
        "3" => show_all_employees(&departments),
        _ => {
            departments = handle_user_options(departments);
        }
    }

    departments
}

fn add_employee_process(
    mut departments: HashMap<DepartmentStyle, Department>,
) -> HashMap<DepartmentStyle, Department> {
    let (new_emp_dept, new_emp_name) = add_employee();
    let dept = departments.get_mut(&new_emp_dept).unwrap();
    dept.employees.push(new_emp_name);
    departments
}

fn add_employee() -> (DepartmentStyle, String) {
    println!("Add <User Name> to <Department Name>:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    // println!("{user_input}");

    let mut employee_name = String::from("");
    let mut department_name = String::from("");
    let mut user_name_phase = false;
    let mut department_name_phase = false;

    for word in user_input.split_whitespace() {
        if word.to_lowercase() == String::from("add") {
            user_name_phase = true;
        } else if word.to_lowercase() == String::from("to") {
            user_name_phase = false;
            department_name_phase = true;
        } else {
            if user_name_phase {
                employee_name = vec![employee_name, word.to_string()].join(" ");
            } else if department_name_phase {
                department_name = vec![department_name, word.to_string()].join(" ");
            }
        }
    }

    // println!("{:?}", department_name.as_str());
    let department_style: DepartmentStyle =
        get_department_style_from_string(String::from(department_name.trim()));

    // println!("{:?}: {}", department_style, employee_name);

    (department_style, employee_name)
}

fn show_employees_by_department(
    departments: &HashMap<DepartmentStyle, Department>,
    style: DepartmentStyle,
) {
    let dept = departments.get(&style).cloned().unwrap();
    println!("\n{:?} Department Employees:", style);
    let mut employees = dept.employees;
    employees.sort();
    for emp in employees {
        println!("\t - {}", emp);
    }
}

fn show_all_employees(departments: &HashMap<DepartmentStyle, Department>) {
    for style in DepartmentStyle::iter() {
        show_employees_by_department(&departments, style)
    }
}

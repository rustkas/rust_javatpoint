/*
cargo run -p structs --bin struct
cargo fmt --verbose --package structs
*/

#[derive(Debug)]
struct Student {}

#[derive(Debug)]
struct Employee {
    employee_name: String,
    employee_id: u64,
    employee_profile: String,
    active: bool,
}

fn create_employee1(name: String, profile: String) -> Employee {
    Employee {
        employee_name: name,
        employee_id: 12,
        employee_profile: profile,
        active: true,
    }
}

fn create_employee2(employee_name: String, employee_profile: String) -> Employee {
    Employee {
        employee_name,
        employee_id: 12,
        employee_profile,
        active: true,
    }
}

fn main() {
    let student = Student {};
    println!("{:?}", student);

    let employee = Employee {
        employee_name: String::from("Akshay Gupta"),
        employee_id: 12,
        employee_profile: String::from("Computer Engineer"),
        active: true,
    };
    println!("{:?}", employee);

    let employee1 = create_employee1("Akshay Gupta".to_string(), "Computer Engineer".to_string());
    let employee2 = create_employee2("Akshay Gupta".to_string(), "Computer Engineer".to_string());
    println!("{:?}", employee1);
    println!("{:?}", employee2);
}

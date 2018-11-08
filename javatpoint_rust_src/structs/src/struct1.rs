/*
cargo run -p structs --bin struct1
cargo fmt --verbose --package structs
*/

#[derive(Debug)]
struct Employee {
    employee_name: String,
    employee_id: u64,
    employee_profile: String,
    active: bool,
}

fn main() {
    let mut employee1 = Employee {
        employee_name: String::from("Akshay Gupta"),
        employee_id: 12,
        employee_profile: String::from("Computer Engineer"),
        active: true,
    };
    println!("{:?}", employee1);
    {
        let employee2 = Employee {
            employee_name: String::from("Akhil Gupta"),
            employee_id: 11,
            employee_profile: employee1.employee_profile,
            active: employee1.active,
        };
        println!("{:?}", employee2);
    }
    employee1 = Employee {
        employee_name: String::from("Akshay Gupta"),
        employee_id: 12,
        employee_profile: String::from("Computer Engineer"),
        active: true,
    };
    {
        let employee2 = Employee {
            employee_name: String::from("Akhil Gupta"),
            employee_id: 11,
            ..employee1
        };
        println!("{:?}", employee2);
    }
}

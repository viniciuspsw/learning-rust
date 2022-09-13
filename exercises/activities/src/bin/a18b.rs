// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeRole {
  Maintenance,
  Marketing,
  Manager,
  LineSupervisor,
  Kitchen,
  Assembly
}

struct Employee {
  role: EmployeeRole,
  is_active: bool
}

fn check_active(employee: &Employee) -> Result<(), String> {
  match employee.is_active {
    true => Ok(()),
    false => Err("forbidden access due employee not active".to_owned())
  }
}

fn check_role(employee: &Employee) -> Result<(), String> {
  match employee.role {
    EmployeeRole::Maintenance => Ok(()),
    EmployeeRole::Marketing => Ok(()),
    EmployeeRole::Manager => Ok(()),
    _ => Err("forbidden access due role restrictions".to_owned())
  }
}

fn has_access(employee: &Employee) -> Result<String, String> {
  check_active(employee)?;
  check_role(employee)?;
  Ok("everything ok".to_owned())
}

fn main() {
  let employee = Employee { role: EmployeeRole::Manager, is_active: true };
  match has_access(&employee) {
    Ok(message) => println!("Access allowed: {:?}", message),
    Err(err) => println!("Access denied: {:?}", err)
  }
}

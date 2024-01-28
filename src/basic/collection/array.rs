pub fn main() {
    let students = ["Alex", "John", "Mike"];
    let student_default = "<no name>";

    println!("{}", students.get(0).unwrap());
    println!("{}", students.get(1).unwrap());
    println!("{}", students.get(2).unwrap());
    println!("{}", students.get(3).unwrap_or(&student_default));
    println!("{}", students.get(4).unwrap_or(&student_default));
    println!("{:?}", students);
}

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("student_details.csv")?;

    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    // Write header row
    file.write_all(b"Student Name,Matric. Number,Department,Level\n")?;

    // Write student data
    for (name, matric_no, department, level) in students {
        file.write_all(format!("{},{},{},{}\n", name, matric_no, department, level).as_bytes())?;
    }

    Ok(())
}
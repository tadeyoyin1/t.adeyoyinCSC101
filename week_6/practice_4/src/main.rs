fn main() {
    let fullname = "Timilehin NWose Adeyoyin";
    let department = "Software engineering";
    let uni = "Pan-Atlantic University";

    let mut school = "School of science".to_string();

    school.push_str(" and Technology");

    println!("The length my fullname is {}",fullname.len() );
    println!("I am a student of {} department", department);
    println!("{}", school);
    println!("{}", uni);

}

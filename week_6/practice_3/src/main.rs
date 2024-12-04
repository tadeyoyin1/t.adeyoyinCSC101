fn main() {
    let name1 = "Ayomide Adesokan";
    println!("My name is {}", name1);

    let name2 = name1.replace("Ayomide","Clown");
    println!("You can also call me {}", name2);
    let faculty = "Faculty of science and technology";

    let school = faculty.replace("Faculty","school");
    println!("I am a student of the {}", school);
}

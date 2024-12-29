fn main() {
    let mut staff_levels = vec![
        vec!["Intern", "-", "Paralegal", "Placement"],
        vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"],
        vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"],
        vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"],
        vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"],
        vec!["CEO", "Dean", "Partner", "Principal"],
    ];

    let mut staff_level_names = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    let mut staff_roles = vec![
        "Office Administrator",
        "Academic",
        "Lawyer",
        "Teacher",
    ];

    let mut staff_level = String::new();
    let mut staff_role = String::new();
    let mut work_experience = String::new();

    println!("Enter staff role:");
    std::io::stdin().read_line(&mut staff_role).expect("Failed to read input");

    println!("Enter work experience (in years):");
    std::io::stdin().read_line(&mut work_experience).expect("Failed to read input");

    let work_experience: u32 = work_experience.trim().parse().expect("Invalid input");

    for i in 0..staff_levels.len() {
        for j in 0..staff_levels[i].len() {
            if staff_levels[i][j] == staff_role.trim() {
                if i == 0 && work_experience < 3 {
                    staff_level = staff_level_names[i].to_string();
                } else if i == 1 && work_experience >= 3 && work_experience <= 5 {
                    staff_level = staff_level_names[i].to_string();
                } else if i == 2 && work_experience >= 5 && work_experience <= 8 {
                    staff_level = staff_level_names[i].to_string();
                } else if i == 3 && work_experience >= 8 && work_experience <= 10 {
                    staff_level = staff_level_names[i].to_string();
                } else if i == 4 && work_experience >= 10 && work_experience <= 13 {
                    staff_level = staff_level_names[i].to_string();
                } else if i == 5 && work_experience > 13 {
                    staff_level = staff_level_names[i].to_string();
                }
            }
        }
    }

    if staff_level.is_empty() {
        println!("Invalid staff role or work experience.");
    } else {
        println!("Staff Level: {}", staff_level);
    }
}
use std::collections::HashMap;

struct Developer {
    name: String,
    years_of_experience: u32,
}

fn main() {
    let mut developers = HashMap::new();

    // Sample data (replace with actual interview data)
    developers.insert(
        "John Doe".to_string(),
        Developer {
            name: "John Doe".to_string(),
            years_of_experience: 5,
        },
    );
    developers.insert(
        "Jane Smith".to_string(),
        Developer {
            name: "Jane Smith".to_string(),
            years_of_experience: 8,
        },
    );
    developers.insert(
        "David Lee".to_string(),
        Developer {
            name: "David Lee".to_string(),
            years_of_experience: 3,
        },
    );

    let mut most_experienced_developer = None;
    let mut max_experience = 0;

    for (name, developer) in &developers {
        if developer.years_of_experience > max_experience {
            max_experience = developer.years_of_experience;
            most_experienced_developer = Some(developer.clone());
        }
    }

    if let Some(developer) = most_experienced_developer {
        println!("Most experienced developer: {:?}", developer);
    } else {
        println!("No developers found.");
    }
}
fn main() {
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    for i in 0..names.len() {
        println!(
            "{} {} {} {} {} {}",
            i + 1,
            names[i],
            i + 1,
            ministries[i],
            i + 1,
            zones[i]
        );
    }
}

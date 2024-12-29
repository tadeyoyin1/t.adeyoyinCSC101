use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("nigerian_breweries.txt")?;

    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    for drink in lager {
        file.write_all(format!("{}\n", drink).as_bytes())?;
    }

    file.write_all(b"\n")?;

    for drink in stout {
        file.write_all(format!("{}\n", drink).as_bytes())?;
    }

    file.write_all(b"\n")?;

    for drink in non_alcoholic {
        file.write_all(format!("{}\n", drink).as_bytes())?;
    }

    Ok(())
}
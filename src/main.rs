
use std::{fs::File, io::{BufRead, BufReader}};


fn main() -> std::io::Result<()> {

    let names = read_clients_name("data/clientes.txt")?;

    for name in &names {
        println!("{}", name);
    }

    let ages = read_clients_age("data/clientes.txt")?;

    for age in &ages {
        println!("{}", age);
    }


    return Ok(())
}


fn read_clients_name(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut names: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("Nombre") {
            let parts: Vec<&str> = line.split(": ").collect();
            let name: String = String::from(parts[1]);

            names.push(name);
        }
    }
    

    return Ok(names);
}

fn read_clients_age(path: &str) -> std::io::Result<Vec<String>> {

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut ages: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("Edad") {
            let parts: Vec<&str> = line.split(": ").collect();
            let age: String = String::from(parts[1]);

            ages.push(age);
        }  
    }


    return Ok((ages))
}
use std::io;

fn main() {
    
    println!("this program culculate your weight in other planet");
    loop {
        println!("0- go out");
        println!("1- venus");
        println!("2- mars");
        println!("3- jupiter");
    
        let mut choise_str = String::new();
        io::stdin().read_line(&mut choise_str).unwrap();
        let choise: i32 = choise_str.trim().parse().unwrap();
    
        if choise == 1 {
            println!("enter your weight in erth");
            let mut weight_in_erth1_str = String::new();
            io::stdin().read_line(&mut weight_in_erth1_str).unwrap();
            let weight_in_erth1: f32 = weight_in_erth1_str.trim().parse().unwrap();
            print!("your weight in venus :");
            weight_in_venus(weight_in_erth1);
        }

        if choise == 2 {
            println!("enter your weight in erth");
            let mut weight_in_m_str = String::new();
            io::stdin().read_line(&mut weight_in_m_str).unwrap();
            let weight_in_m: f32 = weight_in_m_str.trim().parse().unwrap();
            print!("your weight in mars :");
            weight_in_mars(weight_in_m);
        }


        if choise == 3 {
            println!("enter your weight in erth");
            let mut weight_in_e_str = String::new();
            io::stdin().read_line(&mut weight_in_e_str).unwrap();
            let weight_in_e: f32 = weight_in_e_str.trim().parse().unwrap();
            print!("your weight in jupiter :");
            weight_in_jupiter(weight_in_e);
        }
        
        if choise == 0 {
            break;
        }
    }    

}

fn weight_in_mars(inputm: f32) {
    println!("{}", {inputm * 0.38});
}

fn weight_in_jupiter(inputj: f32) {
    println!("{}", {inputj * 2.52});
}

fn weight_in_venus(inputv: f32) {
    println!("{}", {inputv * 0.9});
}    
    

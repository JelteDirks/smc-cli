use macsmc::Smc;

fn main() {
    let mut smc = Smc::connect().unwrap();
    let x = smc.number_of_keys().unwrap();
    println!("{:?}", x);

    let data = smc.all_data().unwrap();
    for d in data {
        println!("{:?}", d);
    }
}

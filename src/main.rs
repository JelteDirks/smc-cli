use macsmc::Smc;

fn main() {
    let mut smc = Smc::connect().unwrap();
    let cpu = smc.cpu_temperature().unwrap();
    println!("cpu {:?}", cpu);
}

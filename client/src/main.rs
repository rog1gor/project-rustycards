mod card;
use card::Minion;
fn main() {
    let mut minion1 = Minion::new("rog1gor".to_string(), 2, 5);
    let mut minion2 = Minion::new("obukaj".to_string(), 1, 3);
    println!("{}", minion1.to_string());
    println!("{}", minion2.to_string());
    minion1.attack(&mut minion2);
    println!("{}", minion1.to_string());
    println!("{}", minion2.to_string());
    minion1.attack(&mut minion2);
    println!("{}", minion1.to_string());
    println!("{}", minion2.to_string());
}

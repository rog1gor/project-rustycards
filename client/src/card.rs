pub struct Minion{
    hp: i32,
    dmg: i32,
    name: String
}

impl Minion {
    pub fn new(name: String, dmg: i32, hp: i32 ) -> Self {
        Minion { hp, dmg, name}
    }

    pub fn is_alive(&self) -> bool{
        return self.hp > 0
    }

    pub fn take_dmg(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    //Function attack damages both attacer and a target
    pub fn attack(&mut self, target: &mut Minion) {
        println!("{} attacked {}", self.name, target.name);
        target.take_dmg(self.dmg);
        self.take_dmg(target.dmg);

        //print if someone died
        if !self.is_alive(){
            println!("{} died", self.name);
        }
        if !target.is_alive(){
            println!("{} died", target.name);
        }
    }

    pub fn to_string(&self) -> String {
        format!("Minion {{ name: {}, dmg: {}, hp: {} }}", self.name, self.dmg, self.hp)
    }
}
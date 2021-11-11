pub struct Chest {
    items: Vec<Item>
}


pub enum Item {
    Sword {
        weapon: bool,
        damage: Option<i32>,
        cooldown: Option<i32>,
        

    },
    
}

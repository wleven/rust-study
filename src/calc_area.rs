#[derive(Debug)]
pub struct React {
    pub width: u32,
    pub height: u32,
}

impl React {
    pub fn calc(&self) -> u32 {
        println!("{:#?}", self);
        self.width * self.height
    }
}

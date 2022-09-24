
#[derive(Debug)]
pub struct React {
    pub width: u32,
    pub height: u32,
}

pub fn calc(_data: &React) -> u32 {
    println!("{:#?}",_data);
    _data.width * _data.height
}

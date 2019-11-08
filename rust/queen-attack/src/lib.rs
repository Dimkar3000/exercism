pub struct ChessPosition(u8,u8);
pub struct Queen(u8,u8);

impl ChessPosition {
    pub fn new(x:i32,y:i32) -> Result<ChessPosition,()>{
        if x < 0 || x > 7 || y < 0 || y>7{
            return Err(());
        }
        Ok(ChessPosition(x as u8,y as u8))
    }

}

impl Queen{

    pub fn new(pos:ChessPosition) -> Queen{
        Queen(pos.0,pos.1)
    }

    pub fn can_attack(&self,other:&Queen) -> bool{
        if self.0 == other.0 || self.1 == other.1 ||  (self.0 as i8 - other.0 as i8).abs() == (self.1 as i8 - other.1 as i8).abs(){
            return true;
        }
        false
    }

}
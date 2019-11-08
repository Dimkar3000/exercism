#[macro_export]
macro_rules! hashmap{
    ($( $x:tt => $y:expr),* $(,)* ) =>{
        {
            let mut hm = HashMap::new();
            $(
                hm.insert($x,$y);   
            )*
            hm
        }
    };
}
pub struct PascalsTriangle{
    data: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut result = PascalsTriangle{data:vec![]};
        let mut size = 1;
        for _ in 0..row_count{
            let mut l = vec![];
            for j in 0..size{
                if j == 0 || j == size-1{
                    l.push(1);
                }
                else{
                    l.push(result.data[result.data.len()-1][j-1] 
                           + result.data[result.data.len()-1][j]);
                }
            }
            result.data.push(l);
            size += 1;
        }
        result
    }
    //Vector implements Clone
    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.data.clone()
    }
}
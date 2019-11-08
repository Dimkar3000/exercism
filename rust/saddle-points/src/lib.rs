pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut colmins= vec![(0xffffffffu64, vec![0usize]); input[0].len()];
    let mut rowmaxes = vec![];
    for (i,row) in input.iter().enumerate(){
        let mut rmax = 0u64;
        let mut rmaxj: Vec<usize> = vec![];
        for (j, (&mut (ref mut cm, ref mut _is), &e)) in colmins.iter_mut().zip(row.iter()).enumerate(){
            if *cm > e {
                *cm = e;
                *_is = vec![i];
            }else if *cm == e {
                _is.push(i);
            }
            if e > rmax {
                rmax = e;
                rmaxj = vec![j];
            }else if e == rmax {
                rmaxj.push(j);
            }
        }
        rowmaxes.push((rmax.clone(),rmaxj.clone()));

    }

    for (j, &(c, ref _is)) in colmins.iter().enumerate(){
        for &i in _is.iter(){
            let (rm, ref rmj) = rowmaxes[i];
            if rm == c && rmj.iter().any(|c| *c ==j){
                result.push((i,j));
            }
        }
    }
    result
}
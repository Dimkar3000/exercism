pub fn find(input:&[i32],key:i32)->Option<usize>
{
    if input.len() == 0 {
        return None;
    }
    if key < input[0] || key > input[(input.len()-1)]{
        return None;
    }
    let mut index = input.len()-1;
    loop{
        if key < input[index]{
            index /=2;
            continue;
        }else if input[index] < key && input[index+1] > key{
            return None;
        }else if key > input[index]{
            index += index/2;
            continue;
        }else if input[index] == key{
            return Some(index);
        }
        else {
            return None;
        }
    }
}
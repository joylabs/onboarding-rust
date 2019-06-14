pub fn exercise9(i:i32)->bool{

    if i< 1{
        return false
    }
    //Bitwise AND &
    if i & (i-1) > 0{
        return false
    }
    true
}
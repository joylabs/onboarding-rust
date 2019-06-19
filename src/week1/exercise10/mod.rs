
pub fn exercise10(v:&mut Vec<&str>){

    let mut j = v.len()-1;
    for i in 0..v.len(){
        if i>j {
            break;
        }
        v.swap(i,j);
        j-=1;
    }
}
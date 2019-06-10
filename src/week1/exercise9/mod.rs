pub fn exercise9(i:i32)->bool{

    let f :f32 = i as f32;
    let f = f.log2();
    let rest = f.floor() - f;

    rest.abs()<1e-10
}
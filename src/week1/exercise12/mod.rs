pub fn exercise12(word:String)->bool{
    let mut word2 = String::from("");
    let mut word3 = String::from("");


    for i in word.split_whitespace() {
        for j in i.chars() {
            if j.is_alphanumeric() {
                word3.push(j);
            }
        }
    }
    word3 = word3.to_lowercase();
    
    for j in word3.chars().rev() {
        word2.push(j);
    }
    word2 == word3
}
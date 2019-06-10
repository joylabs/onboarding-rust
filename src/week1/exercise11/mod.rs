pub fn exercise11(word : String)->String{
    let mut word2 = String::from("");

    for i in word.split_whitespace() {
        for j in i.chars().rev() {
            word2.push(j);
        }
        word2.push_str(" ");
    }
    word2.pop();
    word2
}
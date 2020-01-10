use std::io;
fn main() {
    println!("Please enter String to convert into Pig Latin:");
    let s = get_text();
    let s = pig_latin(s);
    println!("{:#?}",s);
}
fn pig_latin(s:String) -> String {
    //io::stdout().flush().unwrap();
    let mut pl_str = String::new();
    for word in s.split_whitespace(){
        if pl_str.len()>0{
            pl_str.push_str(" ");
        }
        match &word[0..1]{
            "a"|"A"|"e"|"E"|"i"|"I"|"o"|"O"|"u"|"U" =>{
                pl_str.push_str(&word[..]);
                pl_str.push_str("-hay");
            },
            _=> {
                pl_str.push_str(&word[1..word.len()]);
                pl_str.push_str("-");
                pl_str.push_str(&word[0..1]);
                pl_str.push_str("ay");
            }
        }
    }
    pl_str
}
fn get_text()->String{
    loop{
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("Failed to read line!");
        let ln =text.len();
        if ln==0{
            println!("Please enter String to convert into Pig Latin:");
            continue;
        }
        return text;
    }
}

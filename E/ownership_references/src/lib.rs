pub fn inspect(arg: &String){
    if arg.ends_with("s"){
        println!("The word {} is plural", arg);
    }else{
        println!("The word {} is singular", arg)
    }
}

pub fn change(arg: &mut String){
    if !arg.ends_with("s"){
        arg.push_str("s");
    }
}

pub fn eat(arg: String) -> bool{
    if arg.starts_with("b") && arg.contains("a") || arg.starts_with("B") && arg.contains("a")
}

pub fn add(a: &i32, b: &i32) -> i32{
    *a+*b
}

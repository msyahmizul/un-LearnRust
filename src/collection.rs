pub fn main(){
    // let v:Vec<i32>= Vec::new();
    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
    // let mut scores = std::collections::HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: std::collections::HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
}
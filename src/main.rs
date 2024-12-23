use std::env::var;

fn main() {
    let mut data: Vec<String> = Vec::new();
    data.push("test".to_string());
    data.push("randomName".to_string());
    data.push("rand".to_string());
    data.push("Value".to_string());
    data.push("Data".to_string());
    data.push("test v2".to_string());

    let testValue = "Testtttttt";

    wagenr_fischer_distance(testValue.to_string(), data[0].to_string());
}

fn wagenr_fischer_distance(s1: String, s2: String) -> u16 {
    let columns = s1.len();
    let rows = s2.len();

    //  we know that ALWAYS first row looks like this [1,2,3,4,5....]
    let mut previousRow = vec![0; columns];
    for index in 0..columns {
        previousRow[index] = index + 1;
    }

    println!("{:?}", previousRow);
    return 1;
}

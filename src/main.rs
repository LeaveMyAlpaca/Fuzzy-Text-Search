use std::{cmp, f32::consts::E};
fn main() {
    let mut data: Vec<String> = Vec::new();
    data.push("world".to_string());
    data.push("wood".to_string());
    data.push("wild".to_string());
    data.push("rod".to_string());
    data.push("from".to_string());
    data.push("will".to_string());
    data.push("who".to_string());
    data.push("would".to_string());
    data.push("good".to_string());
    data.push("well".to_string());

    let testValue = "wrlod";

    for text in data.iter() {
        let distance = WagnerFischerDistance(testValue.to_string(), text.to_string());
        println!("{} distance: {}", text, distance);
    }
}

fn WagnerFischerDistance(s1: String, s2: String) -> usize {
    let columns = s1.len() + 1;
    let rows = s2.len() + 1;

    let char1: Vec<char> = s1.chars().collect();
    let char2: Vec<char> = s2.chars().collect();

    //* */  we know that ALWAYS first row looks like this [1,2,3,4,5....]
    let mut previousRow = vec![0; columns];
    for index in 0..columns {
        previousRow[index] = index;
    }
    // println!("0 row: {:?}", previousRow);

    //  we skip first row
    for rowIndex in 1..rows {
        let mut currentRow = vec![0; columns];
        currentRow[0] = rowIndex;
        for columnIndex in 1..columns {
            // if the values are the same we can just setMin other wise we need to add one
            let add: usize;

            if char1[columnIndex - 1] == char2[rowIndex - 1] {
                add = 0;
            } else {
                add = 1;
            }
            let y1 = previousRow[columnIndex];
            let x1y1 = previousRow[columnIndex - 1];
            let x1 = currentRow[columnIndex - 1];
            currentRow[columnIndex] = cmp::min(cmp::min(y1, x1y1), x1) + add;
        }
        // println!("{} row: {:?}", rowIndex, currentRow);
        previousRow = currentRow;
    }

    //* */ we take last row's last index and thats our distance!
    return previousRow[previousRow.len() - 1];
}

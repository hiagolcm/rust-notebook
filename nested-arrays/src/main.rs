fn transpose(mut array: [[u8; 3]; 3]) -> [[u8; 3]; 3] {
    for i in 0..3 {
        for j in i + 1..3 {
            let tmp = array[j][i];
            array[j][i] = array[i][j];
            array[i][j] = tmp;
        }
    }

    array
}

fn main() {
    let array: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("{:?}", transpose(array));
}

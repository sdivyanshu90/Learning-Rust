fn transpose(mut matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    for i in 0..3 {
        for j in i + 1..3 {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
    matrix
}

fn main() {
    let matrix = [
        [1, 2, 3], // <-- the comment makes rustfmt add a newline
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("Original:");
    for row in matrix {
        println!("{row:?}");
    }

    let transposed = transpose(matrix);

    println!("\nTransposed:");
    for row in transposed {
        println!("{row:?}");
    }
}
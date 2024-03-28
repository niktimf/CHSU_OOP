use crate::exercises::{
    compare_results, count_iterations, count_odd_length_words, has_comma_dash, read_pair_of_values,
    read_single_value, Matrix, Point,
};

mod exercises;

fn main() {
    // Задание 1
    println!("Введите координаты точки (x, y):");
    let (x, y) = read_pair_of_values::<f64>().unwrap();
    let point = Point::new(x, y);
    if point.belongs_to_shaded_area() {
        println!("Точка принадлежит заштрихованной области");
    } else {
        println!("Точка не принадлежит заштрихованной области");
    }

    // Задание 2
    println!("Введите целое число n:");
    let n: usize = read_single_value().unwrap();
    println!("Введите последовательность");
    let sequence: String = read_single_value().unwrap();
    let sequence: Vec<char> = sequence.trim().chars().collect();
    if has_comma_dash(n, &sequence) {
        println!("В последовательности найдены символы ',' и '-' на соседних позициях");
    } else {
        println!("В последовательности не найдены символы ',' и '-' на соседних позициях");
    }

    // Задание 3
    let x = std::f64::consts::PI / 4.0;
    let n = 10;

    let (cos_series_result, cos_math_result, difference) = compare_results(x, n);

    println!(
        "Значение cos(x) с помощью бесконечного ряда: {}",
        cos_series_result
    );
    println!(
        "Значение cos(x) с помощью математической функции: {}",
        cos_math_result
    );
    println!("Разница: {}", difference);

    // Задание 4
    let n = 11;
    let iterations = count_iterations(n);
    println!("Количество действий для числа {}: {}", n, iterations);

    // Задание 5
    let mut matrix = Matrix::new(vec![
        vec![5, 2, 8, 4],
        vec![2, 9, 3, 7],
        vec![6, 3, 2, 9],
        vec![8, 4, 6, 1],
    ]);

    println!("Матрица до обмена минимальных элементов:{:?}", matrix);

    matrix.swap_min_elements();

    println!("Матрица после обмена минимальных элементов:{:?}", matrix);

    // Задание 6
    let input_string = "Rust is a programming language";
    let count = count_odd_length_words(input_string);
    println!("Количество слов с нечетной длиной: {}", count);
}

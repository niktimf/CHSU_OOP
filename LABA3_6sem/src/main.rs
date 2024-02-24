use crate::exercises::{
    generate_words_with_specific_char, get_initials, get_longest_white_spaced_element_with_length,
    print_digits, read_single_value, EvenNumberFileFinder, FileExamResultsProcessor,
    FileIOProcessor, SquareMatrix,
};
use std::collections::HashMap;
use std::io::BufRead;

mod exercises;

fn main() {
    // Задание 1
    let num: u32 = 123456;
    print_digits(num);

    // Задание 2
    let matrix = SquareMatrix::new(5);
    println!("Матрица: {:?}", matrix);
    let filled_matrix = matrix.fill_matrix_with_random_numbers((10, 99));
    println!(
        "Матрица, заполненная случайными числами в интервале [10,99]: {:?}",
        filled_matrix
    );
    let max_matrix_element_with_index = filled_matrix.find_max_element_with_index().unwrap();
    println!(
        "Максимальный элемент и его индекс: {:?}",
        max_matrix_element_with_index
    );
    let min_matrix_element_with_index = filled_matrix.find_min_element_with_index().unwrap();
    println!(
        "Минимальный элемент и его индекс: {:?}",
        min_matrix_element_with_index
    );

    // Задание 3
    println!("Введите строку:");
    let input_phrase: String = read_single_value().expect("Ошибка ввода!");
    println!(
        "Самое длинное слово и его длина: {:?}",
        get_longest_white_spaced_element_with_length(input_phrase.as_str()).unwrap()
    );

    // Задание 4
    println!("Введите фамилию, имя и отчество:");
    let full_name: String = read_single_value().expect("Hello, World!");
    println!("Инициалы: {}", get_initials(full_name.as_str()));

    // Задание 5
    let alphabet = vec!['Ы', 'Ш', 'Ч', 'О'];
    let fixed_element = 'Ы';
    let word_length = 3;
    let fixed_position = 1;

    let words =
        generate_words_with_specific_char(&alphabet, fixed_element, word_length, fixed_position);
    println!("Слова: {:?}, количество: {}", words, words.len());

    // Задание 6
    let file_with_numbers = FileIOProcessor::new(
        "input_numbers.txt".parse().unwrap(),
        "output_numbers.txt".parse().unwrap(),
    );

    let read_file_numbers = file_with_numbers.read_numbers().unwrap();
    let max_even_number = file_with_numbers
        .find_max_even_number(&read_file_numbers)
        .unwrap();
    println!("Максимальное четное число в файле: {:?}", max_even_number);

    let min_even_number = file_with_numbers
        .find_min_even_number(&read_file_numbers)
        .unwrap();
    println!("Минимальное четное число в файле: {:?}", min_even_number);

    file_with_numbers
        .write_numbers_result(max_even_number, min_even_number)
        .unwrap();

    // Задание 7
    let file_with_exam_results = FileIOProcessor::new(
        "input_exam_results.txt".parse().unwrap(),
        "output_exam_results.txt".parse().unwrap(),
    );

    let read_file_exam_results = file_with_exam_results.read_exam_results().unwrap();
    let filter_exam_results_more_than_80 =
        file_with_exam_results.filter_exam_results(&read_file_exam_results, 80);
    println!(
        "Учащиеся с результатом экзамена более 80 баллов: {:?}",
        filter_exam_results_more_than_80
    );
    file_with_exam_results
        .write_exam_results(&filter_exam_results_more_than_80)
        .unwrap();

    // Задание 8
    let mut counts = HashMap::new();
    loop {
        println!("Введите данные (должность пол количество) или 'stop' для завершения:");
        let input: String = read_single_value().expect("Ошибка ввода!");

        if input == "stop" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() == 3 {
            let gender = parts[1];
            let count: i32 = parts[2].parse().unwrap_or(0);
            *counts.entry(gender.to_string()).or_insert(0) += count;
        } else {
            println!("Неверный формат ввода. Попробуйте снова.");
        }
    }

    println!("man {}", counts.get("man").unwrap_or(&0));
    println!("woman {}", counts.get("woman").unwrap_or(&0));
}

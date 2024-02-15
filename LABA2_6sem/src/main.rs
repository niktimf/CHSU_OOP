use crate::exercises::{fibonacci_numbers_less_than, find_mastic_combinations,
                       has_adjacent_duplicates, has_duplicate_digits_with_hashset,
                       is_armstrong, is_automorphic, is_divisible_by_its_digits,
                       multiply_without_operator, read_pair_of_values, read_single_value,
                       sieve_of_eratosthenes, transform_data};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::cmp::Ordering::{Equal, Greater};
use std::time::Instant;

mod exercises;

fn main() {
    // Задание 1
    println!("Введите два целых числа");
    let (a, b): (u32, u32) = read_pair_of_values().expect("Ошибка парсинга");
    match a.cmp(&b) {
        Greater | Equal => {
            println!("Условие 0 < A < B не выполнено.");
            return;
        }
        _ => {
            (a..=b).for_each(|x| println!("{} * {} = {}", x, x, x * x));
            // for i in a..=b {
            //     println!("{} * {} = {}", i, i, i*i);
            // }
        }
    }


    // Задание 2
    println!("Введите два числа:");
    let (a, b) = read_pair_of_values().expect("Ошибка чтения чисел");

    let product = multiply_without_operator(a, b);
    println!("{} * {} = {}", a, b, product);


    // Задание 3
    println!("Введите число N");
    let n: u64 = read_single_value().expect("Ошибка парсинга");
    println!(
        "Сумма всех чисел Фибоначчи, меньших N: {:?}",
        fibonacci_numbers_less_than(n).iter().sum::<u64>()
    );


    // Задание 4
    println!("Введите натуральное число");
    let n: String = read_single_value().expect("Ошибка парсинга");
    let sum: u32 = n.chars().filter_map(|c| c.to_digit(10)).sum();

    println!("Сумма цифр {}", sum);


    // Задание 5
    println!("Введите натуральное число");
    let n: String = read_single_value().expect("Ошибка парсинга");
    if has_adjacent_duplicates(n) {
        println!("Да");
    } else {
        println!("Нет.");
    }


    // Задание 6
    println!("Введите натуральное число");
    let n: String = read_single_value().expect("Ошибка парсинга");
    if has_duplicate_digits_with_hashset(n) {
        println!("Да");
    } else {
        println!("Нет.");
    }


    // Задание 7
    let start = 10000;
    let end = 99999;
    (start..=end)
        .filter(|&num| num % 133 == 125 && num % 134 == 111)
        .for_each(|num| println!("{}", num));


    // Задание 8
    println!("Трехзначные числа Армстронга:");
    (100..=999).for_each(|x| {
        if is_armstrong(x) {
            println!("{}", x);
        }
    });

    // for num in 100..=999 {
    //     if is_armstrong(num) {
    //         println!("{}", num);
    //     }
    // }


    // Задание 9
    println!("Введите N");
    let n: u64 = read_single_value().expect("Ошибка парсинга");
    (1..=n).for_each(|x| {
        if is_automorphic(x) {
            println!("{} * {} = {}", x, x, x * x);
        }
    });


    // Задание 10
    println!("Введите два целых числа");
    let (a, b): (u32, u32) = read_pair_of_values().expect("Ошибка парсинга");
    match a.cmp(&b) {
        Greater => {
            println!("Условие A < B не выполнено.");
            return;
        }
        _ => {
            // Возвращаем вектор простых чисел до b
            // С помощью filter выбираем только простые числа больше или равные a
            // С помощью for_each выводим на экран простые числа больше или равные a
            let primes = sieve_of_eratosthenes(b as usize);
            primes
                .into_iter()
                .filter(|&p| p >= a as usize)
                .for_each(|p| println!("{}", p));
        }
    }


    // Задание 11
    let crate1_weight = 15;
    let crate2_weight = 17;
    let crate3_weight = 21;
    let total_weight = 185;
    let combinations =
        find_mastic_combinations(crate1_weight, crate2_weight, crate3_weight, total_weight);
    println!("Способы купить {} кг мастики:", total_weight);
    for (x, y, z) in combinations.iter() {
        println!(
            "{}кг: {}, {}кг: {}, {}кг: {}",
            crate1_weight, x, crate2_weight, y, crate3_weight, z
        );
    }
    println!("Всего способов: {}", combinations.len());


    // Задание 12
    println!("Введите N");
    let n: u32 = read_single_value().expect("Ошибка парсинга");
    (1..=n).for_each(|num| {
        if is_divisible_by_its_digits(num) {
            print!("{} ", num);
        }
    });


    // Доп задание
    let data: Vec<i64> = (1..=1000000).collect();

    let start = Instant::now();
    println!("Сумма квадратов чисел, делящихся на 3: {}", transform_data(data));
    let finish = start.elapsed();
    println!("Время выполнения: {:?}", finish);
}

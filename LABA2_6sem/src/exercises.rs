use num_traits::{zero, FromPrimitive, Num, NumCast, PrimInt, Signed};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::cmp::Ordering::Less;
use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::ops::{Add, Mul, Neg, Sub};
use std::str::FromStr;

pub fn read_single_value<T>() -> Result<T, &'static str>
where
    T: FromStr,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");
    input
        .trim()
        .parse::<T>()
        .map_err(|_| "Ошибка парсинга значения")
}

pub fn read_pair_of_values<T: FromStr>() -> Result<(T, T), &'static str>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Необходимо ввести ровно два числа");
    }
    let first = parts[0]
        .parse::<T>()
        .map_err(|_| "Ошибка парсинга первого числа")?;
    let second = parts[1]
        .parse::<T>()
        .map_err(|_| "Ошибка парсинга второго числа")?;
    Ok((first, second))
}

/*
// Задание 2
// Напишите программу, которая получает два целых числа и находит их произведение, не используя операцию умножения. Учтите, что числа могут быть отрицательными.
// Пример:
// Введите два числа:
// 10 -15
// 10 * (-15) = -150
pub fn multiply_without_operator<T>(a: T, b: T) -> T
    where
    T: From<i32> + std::ops::Mul<Output = T> + PartialOrd + std::ops::Neg<Output = T>  {
    let zero = T::from(0);
    match (a, b) {
        (0, _) | (_, 0) => 0,
        (a, b) => {
            let positive_b = if b < 0 { -b } else { b };
            let result = (0..positive_b).fold(0, |acc, _| acc + a);
            // 0 - это начальное значение аккумулятора
            // |acc, _| - это параметры функции агрегации.
            // acc является аккумулятором, который хранит промежуточный результат агрегации.
            // _, представляет каждый элемент итератора,
            // но в данном случае он не используется (что обозначается символом _, указывающим на игнорирование значения),
            // поскольку для агрегации используется только значение a, не зависящее от элементов итератора.
            match b.cmp(&zero) {
                Less => -result,
                _ => result,
            }

            // let mut result = 0;
            // for _ in 0..positive_b {
            //     result += a;
            // }
            // if b < 0 {
            //     - result
            // } else {
            //     result
            // }
        }
    }
}

 */

pub fn multiply_without_operator<T>(a: T, b: T) -> T
where
    T: PrimInt + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + PartialOrd + NumCast,
{
    let zero = T::from(0).expect("Error converting 0 to T");
    let abs_a = if a < zero { -a } else { a };
    let abs_b = if b < zero { -b } else { b };

    // Вычисляем результат умножения, используя абсолютные значения
    let result = (0..T::to_usize(&abs_b).unwrap()).fold(zero, |acc, _| acc + abs_a);
    match (a > zero, b > zero) {
        (true, true) | (false, false) => result,
        (false, true) | (true, false) => -result,
    }
}

// Задание 3
// Ввести натуральное число N и вычислить сумму всех чисел Фибоначчи, меньших N. Предусмотрите защиту от ввода отрицательного числа N.
// Пример:
// Введите число N:
// 10000
// Сумма 17710
pub fn fibonacci_numbers_less_than<T>(n: T) -> Vec<T>
where
    T: Num + Copy + PartialOrd + FromPrimitive,
{
    let mut fibonacci_numbers = Vec::new();
    let zero = T::from_u64(0).unwrap();
    let one = T::from_u64(1).unwrap();
    let (mut a, mut b) = (zero, one);
    while b < n {
        fibonacci_numbers.push(b);
        let next = a + b;
        a = b;
        b = next;
    }

    fibonacci_numbers
}

// Задвние 5
// Ввести натуральное число и определить, верно ли,
// что в его записи есть две одинаковые цифры, стоящие рядом.
// Пример:
// Введите натуральное число:
// 12342
// Нет

// Пример:
// Введите натуральное число:
// 12245
// Да
pub fn has_adjacent_duplicates<T: PartialEq>(sequence: &[T]) -> bool {
    sequence.windows(2).any(|window| window[0] == window[1])
    // let digits = num.chars().collect::<Vec<char>>();
    // for i in 0..digits.len() - 1 {
    //     if digits[i] == digits[i + 1] {
    //         return true;
    //     }
    // }
    // false
}

// Задание 6
// Ввести натуральное число и определить, верно ли,
// что в его записи есть две одинаковые цифры (не обязательно стоящие рядом).
// Пример:
// Введите натуральное число:
// 12342
// Да
//
// Пример:
// Введите натуральное число:
// 12345
// Нет
pub fn has_duplicate_digits_with_hashset<T>(num: &[T]) -> bool
where
    T: Hash + Eq,
{
    let mut seen = HashSet::new();
    num.iter().any(|digit| !seen.insert(digit))
}

pub fn has_duplicate_digits_with_loops<T>(num: &[T]) -> bool
where
    T: Hash + Eq,
{
    num.iter()
        .enumerate()
        .any(|(i, digit)| num.iter().skip(i + 1).any(|next_digit| digit == next_digit))
}

// Задание 8
// Натуральное число называется числом Армстронга, если сумма цифр числа,
// возведенных в N-ную степень (где N – количество цифр в числе) равна самому числу.
// Например, 153 = 13 + 53 + 33.
// Найдите все трёхзначные Армстронга
pub fn is_armstrong(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let sum: u32 = digits.iter().map(|&d| d.pow(3)).sum();
    sum == num
}

// Задание 9
// Натуральное число называется автоморфным, если оно равно последним цифрам своего квадрата.
// Например, 252 = 625.
// Напишите программу, которая получает натуральное число N и выводит на экран все автоморфные числа,
// не превосходящие N.
// Пример:
// Введите N:
// 1000
// 1*1=1
// 5*5=25
// 6*6=36
// 25*25=625
// 76*76=5776
pub fn is_automorphic(num: u64) -> bool {
    let square = num * num;
    // Преобразование числа и его квадрата в строки для сравнения
    let num_str = num.to_string();
    let square_str = square.to_string();

    // Проверка, заканчивается ли строковое представление квадрата числа на строковое представление самого числа
    square_str.ends_with(&num_str)
}

// Задание 10
// Напишите программу, которая получает натуральные числа A и B (A<B) и выводит все простые числа в интервале от A до B.
// Пример:
// Введите границы диапазона:
// 10 20
// 11 13 17 19
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;

    (2..=n).for_each(|i| {
        if sieve[i] {
            // Отмечаем составные числа как не простые
            (i * i..=n).step_by(i).for_each(|j| {
                sieve[j] = false;
            });
        }
    });
    // Оставляем только простые числа
    (2..=n).filter(|&i| sieve[i]).collect()
}

// Задание 11
// В магазине продается мастика в ящиках по 15 кг, 17 кг, 21 кг.
// Как купить ровно 185 кг мастики, не вскрывая ящики?
// Сколькими способами можно это сделать?
pub fn find_mastic_combinations(
    crate1_weight: u32,
    crate2_weight: u32,
    crate3_weight: u32,
    total_weight: u32,
) -> Vec<(u32, u32, u32)> {
    let max_crates1 = total_weight / crate1_weight;
    let max_crates2 = total_weight / crate2_weight;
    let max_crates3 = total_weight / crate3_weight;

    let combinations = (0..=max_crates1)
        .flat_map(move |x| {
            let x_weight = crate1_weight * x;
            //println!("Проверяем x = {}: x_weight = {}", x, x_weight);
            (0..=max_crates2).flat_map(move |y| {
                let y_weight = crate2_weight * y;
                (0..=max_crates3).filter_map(move |z| {
                    let z_weight = crate3_weight * z;
                    if x_weight + y_weight + z_weight == total_weight {
                        Some((x, y, z))
                    } else {
                        None
                    }
                })
            })
        })
        .collect();
    combinations
}

// Задание 12
// Ввести натуральное число N и вывести все натуральные числа, не превосходящие N и делящиеся на каждую из своих цифр.
// Пример:
// Введите N:
// 15
// 1 2 3 4 5 6 7 8 9 11 12 15
pub fn is_divisible_by_its_digits(num: u32) -> bool {
    num.to_string().chars().all(|c| {
        c.to_digit(10)
            .map_or(false, |digit| digit != 0 && num % digit == 0)
    })
}

// Доп задание
pub fn transform_data(data: Vec<i64>) -> i64 {
    data.par_iter() // Используем параллельный итератор
        .filter(|&&x| x % 3 == 0 && x % 5 == 0) // Фильтруем числа, делящиеся на 3 и на 5
        .map(|&x| x * x) // Возводим каждое число в квадрат
        .sum() // Суммируем все квадраты
}

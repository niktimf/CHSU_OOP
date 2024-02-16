mod exercises;

use crate::exercises::{calculate_and_print, calculate_distance, read_input, read_point_coordinates};
use rand::Rng;


fn main() {
    // Часть 1
    // Задание 1
    println!("Введите число a:");
    let a: i32 = read_input().expect("Ошибка парсинга");

    println!("Введите число b:");
    let b: i32 = read_input().expect("Ошибка парсинга");

    calculate_and_print(a, b);


    // Задание 2
    let a = read_point_coordinates("Введите координаты точки A (x y):");
    let b = read_point_coordinates("Введите координаты точки B (x y):");
    let distance = calculate_distance(a, b);
    println!("Длина отрезка AB = {:.3}", distance);


    // Задание 3
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(100..1000); // Генерация случайного трехзначного числа

    // Разделение числа на отдельные цифры
    let hundreds = number / 100;
    let tens = (number / 10) % 10;
    let ones = number % 10;

    println!("Получено число {}.", number);
    println!("Его цифры: {}, {}, {}.", hundreds, tens, ones);
    println!("Сумма: {}", hundreds + tens + ones);


    // Задание 4
    println!("Введите количество белочек (N):");
    let n: i32 = read_input().expect("Ошибка парсинга");

    println!("Введите количество орешков (K):");
    let k: i32 = read_input().expect("Ошибка парсинга");

    let nuts_per_squirrel = k / n; // Количество орешков на одну белочку
    let remaining_nuts = k % n; // Орешки, оставшиеся после деления

    println!("Каждой белочке достанется орешков: {}", nuts_per_squirrel);
    println!("Орешков останется: {}", remaining_nuts);


    // Задание 5
    println!("Введите натуральное число N:");
    let n: i32 = read_input().expect("Ошибка парсинга");

    // Проверяем, четное ли число. Если четное, добавляем 2, если нечетное - 1.
    let next_even = if n % 2 == 0 { n + 2 } else { n + 1 };

    println!("Следующее четное число: {}", next_even);


    // Задание 6
    println!("Введите количество учащихся в первом классе:");
    let class1: i32 = read_input().expect("Ошибка парсинга");
    println!("Введите количество учащихся во втором классе:");
    let class2: i32 = read_input().expect("Ошибка парсинга");
    println!("Введите количество учащихся в третьем классе:");
    let class3: i32 = read_input().expect("Ошибка парсинга");

    // Вычисляем количество парт для каждого класса
    let desks_needed = |students: i32| (students + 1) / 2;

    let total_desks = desks_needed(class1) + desks_needed(class2) + desks_needed(class3);

    println!(
        "Минимальное количество парт, которое нужно приобрести: {}",
        total_desks
    );


    // Задание 7
    println!("Введите стоимость пирожка (рубли):");
    let rubles: u32 = read_input().expect("Ошибка парсинга");
    println!("Введите стоимость пирожка (копейки):");
    let cents: u32 = read_input().expect("Ошибка парсинга");
    println!("Введите количество пирожков:");
    let count: u32 = read_input().expect("Ошибка парсинга");

    let total_cents = (rubles * 100 + cents) * count;
    let total_rubles = total_cents / 100;
    let remaining_cents = total_cents % 100;

    println!(
        "Всего нужно заплатить: {} рублей {} копеек",
        total_rubles, remaining_cents
    );


    // Задание 8
    println!("Введите количество секунд:");
    let seconds_total: i32 = read_input().expect("Ошибка парсинга");

    let hours = (seconds_total / 3600) % 24;
    let minutes = (seconds_total % 3600) / 60;
    let seconds = seconds_total % 60;

    println!("{:02}:{:02}:{:02}", hours, minutes, seconds);


    // Задание 9
    println!("Введите часы, минуты и секунды первого момента времени:");
    println!("Введите часы");
    let hours1: i32 = read_input().expect("Ошибка парсинга");
    println!("Введите минуты");
    let minutes1: i32 = read_input().expect("Ошибка парсинга");
    println!("Введите секунды");
    let seconds1: i32 = read_input().expect("Ошибка парсинга");

    println!("Введите часы, минуты и секунды второго момента времени:");
    println!("Введите часы");
    let hours2: i32 = read_input().expect("Ошибка парсинга");
    println!("Введите минуты");
    let minutes2: i32 = read_input().expect("Ошибка парсинга");
    println!("Введите секунды");
    let seconds2: i32 = read_input().expect("Ошибка парсинга");

    let time1_in_seconds = hours1 * 3600 + minutes1 * 60 + seconds1;
    let time2_in_seconds = hours2 * 3600 + minutes2 * 60 + seconds2;

    let difference = time2_in_seconds - time1_in_seconds;
    println!("Секунд между моментами времени: {}", difference);


    // Задание 10
    println!("Введите скорость V:");
    let v: i32 = read_input().expect("Ошибка при парсинге");

    println!("Введите время T:");
    let t: i32 = read_input().expect("Ошибка при парсинге");

    let mkad_length = 109;
    let position = ((v * t) % mkad_length + mkad_length) % mkad_length;
    println!("Номер отметки, на которой остановится Вася: {}", position);


    // Задание 11
    println!("Введите четырехзначное число:");
    let num: u32 = read_input().expect("Ожидалось целое число");

    // Преобразуем число в строку и дополняем нулями слева до 4 знаков
    let num_str = format!("{:04}", num);

    // Проверяем, является ли строка симметричной
    match num_str.chars().nth(0) == num_str.chars().nth(3)
        && num_str.chars().nth(1) == num_str.chars().nth(2)
    {
        true => println!("1"),
        false => println!("37"),
    }


    // Задание 12
    println!("Введите высоту шеста (H):");
    let h: i32 = read_input().expect("Ошибка при парсинге");
    println!("Введите высоту подъема за день (A):");
    let a: i32 = read_input().expect("Ошибка при парсинге");
    println!("Введите высоту спуска за ночь (B):");
    let b: i32 = read_input().expect("Ошибка при парсинге");

    // Вычисляем количество дней
    let days = ((h - b - 1) / (a - b)) + 1; // Используем -1 для корректного округления вверх
    println!("Улитка достигнет вершины шеста на {} день.", days);



    // Часть 2
    // Задание 1 и 2
    println!("Сколько целых чисел вы хотите ввести?");
    let count: usize = read_input().expect("Ошибка при парсинге");

    let mut numbers: Vec<i32> = Vec::new();
    for i in 1..=count {
        println!("Введите {} целое число:", i);
        numbers.push(read_input().expect("Ошибка при парсинге"));
    }

    let max_num = numbers.iter().max().unwrap();
    println!("Максимальное число: {}", max_num);


    // Задание 3
    println!("Введите возраст Антона:");
    let age_anton: i32 = read_input().expect("Ошибка при парсинге");
    println!("Введите возраст Бориса:");
    let age_boris: i32 = read_input().expect("Ошибка при парсинге");
    println!("Введите возраст Виктора:");
    let age_viktor: i32 = read_input().expect("Ошибка при парсинге");

    match (age_anton, age_boris, age_viktor) {
        (a, b, c) if a > b && a > c => println!("Антон старше всех"),
        (a, b, c) if b > a && b > c => println!("Борис старше всех"),
        (a, b, c) if c > a && c > b => println!("Виктор старше всех"),
        (a, b, c) if a == b && a > c => println!("Антон и Борис старше Виктора"),
        (a, b, c) if a == c && a > b => println!("Антон и Виктор старше Бориса"),
        (a, b, c) if b == c && b > a => println!("Борис и Виктор старше Антона"),
        _ => println!("Никто не старше"),
    }


    // Задание 4
    println!("Введите три целых числа:");

    let a: i32 = read_input().expect("Ошибка при парсинге");
    let b: i32 = read_input().expect("Ошибка при парсинге");
    let c: i32 = read_input().expect("Ошибка при парсинге");

    match (a, b, c) {
        (a, b, c) if a == b && a == c => println!("Все числа совпадают: {}", 3),
        (a, b, c) if a == b || a == c || b == c => println!("Два числа совпадают: {}", 2),
        _ => println!("Ни одно число не совпадает: {}", 0),
    }


    // Задание 5
    let month: i32 = read_input().expect("Ошибка парсинга");
    match month {
        1 => println!("Зима"),
        2 => println!("Зима"),
        3 => println!("Весна"),
        4 => println!("Весна"),
        5 => println!("Весна"),
        6 => println!("Лето"),
        7 => println!("Лето"),
        8 => println!("Лето"),
        9 => println!("Осень"),
        10 => println!("Осень"),
        11 => println!("Осень"),
        12 => println!("Зима"),
        _ => println!("Неверный номер месяца"),
    }


    // Задание 6
    println!("Введите возраст:");
    let age: u32 = read_input().expect("Ошибка парсинга");

    let declension = match age % 10 {
        11..=19 => "лет",
        1 => "год",
        2..=4 => "года",
        _ => "лет",
    };

    // Вывод результата
    println!("Вам {} {}", age, declension);


    // Задание 7
    println!("Веедите первую сторону треугольника:");
    let a: u32 = read_input().expect("Ошибка парсинга");
    println!("Веедите вторую сторону треугольника:");
    let b: u32 = read_input().expect("Ошибка парсинга");
    println!("Введите третью сторону треугольника:");
    let c: u32 = read_input().expect("Ошибка парсинга");

    if a + b > c && a + c > b && b + c > a && (a != b || b != c || c != a) {
        println!("YES");
    } else {
        println!("NO");
    }


    // Задание 8
    println!("Введите количество глаз");
    let eyes: u32 = read_input().expect("Ошибка парсинга");
    println!("Введите количество ног");
    let legs: u32 = read_input().expect("Ошибка парсинга");

    match (eyes, legs) {
        (eyes, legs) if eyes > 100 && legs == 0 => println!("морской гребешок"),
        (eyes, legs) if eyes == 8 && legs == 8 => println!("паук"),
        (eyes, legs) if eyes == 2 && legs == 4 => println!("кошка"),
        (eyes, legs) if eyes == 2 && legs == 6 => println!("жук"),
        (_, _) => println!("unidentified animal species"),
    }
}

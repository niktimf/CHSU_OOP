use std::io;


pub fn read_input<T>() -> Result<T, T::Err>
where
    T: std::str::FromStr,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");
    input.trim().parse::<T>()
}

// Задание 1
// На вход программе подаются два целых числа a и b. Напишите программу, которая выводит:
// •	сумму чисел a и b;
// •	разность чисел a и b;
// •	произведение чисел a и b;
// •	целую часть от деления числа a на b;
// •	остаток от деления числа a на b;
// •	корень квадратный из суммы их 10-х степеней*

pub fn calculate_and_print(a: i32, b: i32) {
    println!("Сумма чисел a и b: {}", a + b);
    println!("Разность чисел a и b: {}", a - b);
    println!("Произведение чисел a и b: {}", a * b);

    if b != 0 {
        println!("Целая часть от деления числа a на b: {}", a / b);
        println!("Остаток от деления числа a на b: {}", a % b);
    } else {
        println!("Деление на ноль невозможно.");
    }

    let sum_of_powers = (a.pow(10) as f64) + (b.pow(10) as f64);
    println!(
        "Корень квадратный из суммы их 10-х степеней: {}",
        sum_of_powers.sqrt()
    );
}

// Задание 2
// Ввести с клавиатуры координаты двух точек (A и B) на плоскости (вещественные числа).
// Вычислить длину отрезка AB.
// Пример:
// Введите координаты точки A:
// 5.5 3.5
// Введите координаты точки B:
// 1.5 2
// Длина отрезка AB = 4.272

pub fn read_point_coordinates(prompt: &str) -> (f64, f64) {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let coords: Vec<&str> = input.trim().split_whitespace().collect();
    if coords.len() != 2 {
        panic!("Неверный формат ввода. Необходимо ввести два вещественных числа.");
    }
    let x = coords[0]
        .parse::<f64>()
        .expect("Неверный формат ввода. Необходимо ввести вещественное число.");
    let y = coords[1]
        .parse::<f64>()
        .expect("Неверный формат ввода. Необходимо ввести вещественное число.");
    (x, y)
}

pub fn calculate_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (x1, y1) = a;
    let (x2, y2) = b;
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

pub fn read_single_value<T>() -> Result<T, &'static str>
where
    T: FromStr,
    T::Err: Debug,
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
    <T as FromStr>::Err: Debug,
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

// Задание 1
// Требуется написать программу, которая вводит с клавиатуры координаты точки на
// плоскости (x, y – вещественные числа) и определяет принадлежность точки заштрихованной
// области, включая ее границы.
// При составлении таблицы контрольных примеров отдельно пропишите проверку
// попадания точки в области, обозначенные на рисунке заглавными латинскими буквами
// (количество тестов совпадает с количеством заглавных букв).

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn belongs_to_shaded_area(&self) -> bool {
        self.is_inside_circle() && self.is_in_shaded_region()
    }

    fn is_inside_circle(&self) -> bool {
        self.x * self.x + self.y * self.y <= 1.0
    }

    fn is_in_shaded_region(&self) -> bool {
        (self.x >= 0.0 && self.y >= 0.0 && self.y >= self.x)
            || (self.x <= 0.0 && self.y >= 0.0)
            || (self.x <= 0.0 && self.y <= 0.0)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn point_A() {
        let point = Point::new(-0.8, -0.2);
        assert_eq!(point.belongs_to_shaded_area(), true);
    }

    #[test]
    fn point_B() {
        let point = Point::new(-0.5, 0.5);
        assert_eq!(point.belongs_to_shaded_area(), true);
    }

    #[test]
    fn point_C() {
        let point = Point::new(0.4, 0.4);
        assert_eq!(point.belongs_to_shaded_area(), true);
    }

    #[test]
    fn point_D() {
        let point = Point::new(0.6, 0.2);
        assert_eq!(point.belongs_to_shaded_area(), false);
    }

    #[test]
    fn point_E() {
        let point = Point::new(0.5, -0.5);
        assert_eq!(point.belongs_to_shaded_area(), false);
    }

    #[test]
    fn point_F() {
        let point = Point::new(-0.8, 0.6);
        assert_eq!(point.belongs_to_shaded_area(), true);
    }

    #[test]
    fn point_H() {
        let point = Point::new(0.6, 0.8);
        assert_eq!(point.belongs_to_shaded_area(), true);
    }

    #[test]
    fn point_I() {
        let point = Point::new(0.8, 0.6);
        assert_eq!(point.belongs_to_shaded_area(), false);
    }

    #[test]
    fn point_J() {
        let point = Point::new(0.8, -0.6);
        assert_eq!(point.belongs_to_shaded_area(), false);
    }
}

// Задание 2
// Даны натуральное число n, символы S_1,.... S_n.
// Выяснить, имеются ли в последовательности S_1,.... S_n,
// такие члены последовательности S_i, S_i+1, что S_i это запятая, а S_i+1 это тире
pub fn has_comma_dash(n: usize, sequence: &[char]) -> bool {
    if n != sequence.len() {
        return false;
    }

    sequence.windows(2).any(|w| w[0] == ',' && w[1] == '-')
}

// Задание 3
// В приведенных вариантах заданий вычислить значения функций с помощью
// бесконечного ряда и с помощью соответствующих математических функций. Сравнить результаты.
// cos(x) = 1 - x^2/2! + x^4/4! - x^6/6! + ...
pub fn cos_series(x: f64, n: usize) -> f64 {
    (0..n)
        .map(|i| {
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            let term = x.powi(2 * i as i32) / (1..=i).fold(1.0, |acc, j| acc * (2 * j) as f64);
            sign * term
        })
        .sum()
}

pub fn cos_math(x: f64) -> f64 {
    x.cos()
}

pub fn compare_results(x: f64, n: usize) -> (f64, f64, f64) {
    let cos_series_result = cos_series(x, n);
    let cos_math_result = cos_math(x);
    let difference = (cos_series_result - cos_math_result).abs();
    (cos_series_result, cos_math_result, difference)
}

// Задание 4
// Из заданного числа вычли сумму его цифр. Из результата вновь вычли сумму его цифр и т.д.
// Сколько таких действий надо произвести, чтобы получить нуль?

pub fn sum_of_digits(n: i32) -> i32 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum()
}

pub fn count_iterations(mut n: i32) -> i32 {
    (0..)
        .take_while(|_| {
            n = n - sum_of_digits(n);
            n > 0
        })
        .count() as i32
        + 1
}

// Задание 5
// Дан двумерный массив целых чисел из 4 столбцов и 4 строк. Найти минимальный
// элемент в первой половине массива (просмотр вести по строкам) и во второй половине
// массива. Поменять местами эти минимальные элементы массива.

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T: Copy + PartialOrd + Ord + Display> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Matrix { data }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data[0].len()
    }

    fn find_min_pos(&self, start_row: usize, end_row: usize) -> (usize, usize) {
        (start_row..end_row)
            .flat_map(|i| (0..self.cols()).map(move |j| (i, j)))
            .min_by_key(|&(i, j)| self.data[i][j])
            .unwrap()
    }

    pub fn swap_min_elements(&mut self) {
        let min_pos_first_half = self.find_min_pos(0, self.rows() / 2);
        let min_pos_second_half = self.find_min_pos(self.rows() / 2, self.rows());

        let temp = self.data[min_pos_first_half.0][min_pos_first_half.1];
        self.data[min_pos_first_half.0][min_pos_first_half.1] =
            self.data[min_pos_second_half.0][min_pos_second_half.1];
        self.data[min_pos_second_half.0][min_pos_second_half.1] = temp;
    }
}

// Задание 6
// Дана строка. Определить количество слов, длина которых равняется нечетному числу
pub fn count_odd_length_words(s: &str) -> u32 {
    s.split_whitespace()
        .filter(|word| word.len() % 2 != 0)
        .count() as u32
}

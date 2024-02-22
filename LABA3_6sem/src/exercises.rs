use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;
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

// Задание 1
// Напишите процедуру, которая выводит на экран в столбик все цифры переданного ей числа, начиная с последней.
pub fn print_digits<T>(num: T)
where
    T: ToString,
{
    let str_num = num.to_string();
    str_num.chars().rev().for_each(|ch| println!("{}", ch));
}

// Задание 2
// Напишите программу, которая заполняет квадратную матрицу случайными числами в интервале [10,99],
// и находит максимальный и минимальный элементы в матрице и их индексы.

#[derive(Debug)]
pub struct SquareMatrix<T> {
    size: usize,
    matrix_data: Vec<Vec<T>>,
}

impl<T> SquareMatrix<T>
where
    T: Default + Clone,
{
    pub fn new(size: usize) -> Self {
        let matrix_data = vec![vec![T::default(); size]; size];
        Self { size, matrix_data }
    }

    pub fn fill_matrix_with_random_numbers(mut self, range: (T, T)) -> Self
    where
        T: Clone + SampleUniform + PartialOrd,
    {
        let mut rng = thread_rng();
        let mut uniform = Uniform::from(range.0..=range.1);

        self.matrix_data.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|elem| {
                *elem = uniform.sample(&mut rng);
            })
        });
        self
    }

    pub fn find_max_element_with_index(&self) -> Option<(T, (usize, usize))>
    where
        T: PartialOrd + Copy,
    {
        self.matrix_elements_with_indexes(self.matrix_data.as_slice())
            .max_by(|(value_a, _), (value_b, _)| value_a.partial_cmp(value_b).unwrap_or(Equal))
    }

    pub fn find_min_element_with_index(&self) -> Option<(T, (usize, usize))>
    where
        T: PartialOrd + Copy,
    {
        self.matrix_elements_with_indexes(self.matrix_data.as_slice())
            .min_by(|(value_a, _), (value_b, _)| value_a.partial_cmp(value_b).unwrap_or(Equal))
    }

    // 'a обозначает время жизни ссылок внутри матрицы, передаваемой в функцию.
    // Это гарантирует, что данные, на которые ссылается матрица, будут жить как минимум столько же,
    // сколько и время жизни 'a.
    fn matrix_elements_with_indexes<'a>(
        &'a self,
        matrix: &'a [Vec<T>],
    ) -> impl Iterator<Item = (T, (usize, usize))> + 'a
    where
        T: Copy,
    {
        matrix.iter().enumerate().flat_map(move |(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, &value)| (value, (i, j)))
        })
    }
}

// Задание 3
// Ввести с клавиатуры символьную строку и найдите самое длинное слово и его длину.
// Словом считается последовательности непробельных символов,
// отделенная с двух сторон пробелами (или стоящая с краю строки).
// Слова могут быть разделены несколькими пробелами, в начале и в конце строки тоже могут быть пробелы.

pub fn get_longest_white_spaced_element_with_length(s: &str) -> Option<(&str, usize)> {
    s.split_whitespace()
        .map(|word| (word, word.len()))
        .max_by_key(|&(_, len)| len)
}

// Задание 4
// Ввести с клавиатуры в одну строку фамилию, имя и отчество, разделив их пробелом.
// Вывести фамилию и инициалы.

pub fn get_initials(full_name: &str) -> String {
    let parts: Vec<&str> = full_name.split_whitespace().collect();
    match parts.len().cmp(&3) {
        Equal | Greater => format!(
            "{} {}.{}.",
            parts[0],
            parts[1].chars().next().unwrap(),
            parts[2].chars().next().unwrap()
        ),
        _ => "Введите данные в формате: Фамилия Имя Отчество".to_string(),
    }
}

// Задание 5
// В алфавите языке племени «тумба-юмба» четыре буквы: «Ы», «Ш», «Ч» и «О».
// Нужно вывести на экран все возможные слова, состоящие из K букв,
// в которых вторая буква «Ы». Подсчитайте количество таких слов.
pub fn generate_words_with_specific_char<T>(
    alphabet: &[T],
    specific_char: T,
    word_length: usize,
    position: usize,
) -> Vec<String>
where
    T: Clone + Copy + ToString,
{
    if word_length == 0 || position >= word_length {
        return vec![];
    }
    let alphabet_len = alphabet.len();
    let total_words = alphabet_len.pow(word_length as u32 - 1);

    (0..total_words)
        .map(|i| {
            let mut word: Vec<T> = vec![specific_char; word_length];
            let mut current = i;
            (0..word_length).filter(|&j| j != position).for_each(|j| {
                word[j] = alphabet[current % alphabet_len];
                current /= alphabet_len;
            });
            word
        })
        .map(|word| word.into_iter().map(|c| c.to_string()).collect::<String>())
        .collect()
}

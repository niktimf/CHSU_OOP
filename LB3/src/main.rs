use std::collections::HashSet;
use std::fmt;
use std::ops::{Add, Index, IndexMut, Mul};

// Структура для представления массива строк
#[derive(Debug, Clone)]
struct StringArray {
    strings: Vec<String>,
}

impl StringArray {
    // Конструктор по умолчанию
    fn new() -> Self {
        StringArray {
            strings: Vec::new(),
        }
    }

    // Конструктор из вектора строк
    fn from_vec(strings: Vec<String>) -> Self {
        StringArray { strings }
    }

    // Метод для поэлементной конкатенации двух массивов
    fn concat(&self, other: &StringArray) -> StringArray {
        let mut result = self.strings.clone();
        result.extend(other.strings.iter().cloned());
        StringArray::from_vec(result)
    }

    // Метод для упорядочения строк в лексикографическом порядке
    fn sort(&self) -> StringArray {
        let mut sorted = self.strings.clone();
        sorted.sort();
        StringArray::from_vec(sorted)
    }

    // Метод для слияния двух массивов с удалением повторяющихся строк
    fn merge(&self, other: &StringArray) -> StringArray {
        let mut merged_set: HashSet<String> = self.strings.iter().cloned().collect();
        merged_set.extend(other.strings.iter().cloned());
        let merged_vec: Vec<String> = merged_set.into_iter().collect();
        StringArray::from_vec(merged_vec)
    }

    // Метод для вывода на экран всего массива
    fn print(&self) {
        for s in &self.strings {
            println!("{}", s);
        }
    }

    // Метод для вывода на экран заданной строки
    fn print_string(&self, index: usize) {
        if let Some(s) = self.strings.get(index) {
            println!("{}", s);
        } else {
            println!("Index out of bounds");
        }
    }
}

// Перегрузка операции сложения для класса StringArray
impl Add for StringArray {
    type Output = StringArray;

    fn add(self, other: StringArray) -> StringArray {
        self.concat(&other)
    }
}

// Перегрузка операции умножения для класса StringArray
impl Mul<usize> for StringArray {
    type Output = StringArray;

    fn mul(self, rhs: usize) -> StringArray {
        let repeated_strings = self
            .strings
            .iter()
            .cloned()
            .cycle()
            .take(self.strings.len() * rhs)
            .collect();
        StringArray::from_vec(repeated_strings)
    }
}

// Перегрузка операции индексирования для класса StringArray
impl Index<usize> for StringArray {
    type Output = String;

    fn index(&self, index: usize) -> &String {
        &self.strings[index]
    }
}

// Перегрузка операции присваивания для класса StringArray
impl IndexMut<usize> for StringArray {
    fn index_mut(&mut self, index: usize) -> &mut String {
        &mut self.strings[index]
    }
}

// Реализация отображения класса StringArray
impl fmt::Display for StringArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for s in &self.strings {
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

// Функция, которая выполняет слияние объектов и лексикографическое упорядочение строк
fn merge_and_sort(arrays: &[StringArray]) -> StringArray {
    arrays
        .iter()
        .fold(StringArray::new(), |acc, array| acc.merge(array))
        .sort()
}

fn main() {
    let mut arr1 = StringArray::from_vec(vec![
        "Hello".to_string(),
        "World".to_string(),
        "Rust".to_string(),
    ]);
    let arr2 = StringArray::from_vec(vec![
        "Programming".to_string(),
        "Language".to_string(),
        "Rust".to_string(),
    ]);

    println!("Объединение string_array1 и string_array2:");
    let concatenated_array = arr1.concat(&arr2);
    concatenated_array.print();

    println!("\nСортировка string_array1:");
    let sorted_array = arr1.sort();
    sorted_array.print();

    println!("\nВывод string_array1:");
    arr1.print();

    let merged = merge_and_sort(&[arr1.clone(), arr2.clone()]);
    println!("Слияние и сортировка:");
    println!("{}", merged);

    let sum = arr1.clone() + arr2.clone();
    let product = arr1.clone() * 3;
    println!("Сложение: {}", sum);
    println!("Умножение: {}", product);

    println!("Элемент с индексом 1: {}", arr1[1]);
    arr1[1] = "Modified".to_string();
    println!("Модифицированный элемент с индексом 1: {}", arr1[1]);
}

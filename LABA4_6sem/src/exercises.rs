use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::path::PathBuf;
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

// Задание 2
// Напишите функции, которые сортируют значения переданного ей динамического массива,
// используя алгоритмы:
// А) Метод пузырька (сортировка обменами).
// Сначала сравниваем последний элемент с предпоследним.
// Если они стоят неправильно (меньший элемент «ниже»), то меняем их местами.
// Далее так же рассматриваем следующую пару элементов и т.д.
// Б) Метод выбора. На каждом этапе выбирается минимальный элемент (из оставшихся) и ставится на свое место
pub fn bubble_sort<T: Ord>(array: &mut [T]) {
    (0..array.len()).for_each(|i| {
        (0..array.len() - i - 1).for_each(|j| {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        });
    });
}

pub fn selection_sort<T: Ord>(array: &mut [T]) {
    (0..array.len()).for_each(|i| {
        let min_index = (i..array.len()).min_by_key(|&j| &array[j]).unwrap();
        array.swap(i, min_index);
    });
}

// Задание 3
// В файле находится список слов, среди которых есть повторяющиеся.
// Каждое слово записано в отдельной строке.
// Построить алфавитно-частотный словарь: список слов в алфавитном порядке,
// справа от каждого слова должно быть указано, сколько раз оно встречается в исходном файле.
#[derive(Debug)]
pub struct TextFileReader {
    pub input_path: PathBuf,
}

impl TextFileReader {
    // Конструктор для создания нового экземпляра NumberProcessor
    pub fn new(input_path: PathBuf) -> Self {
        TextFileReader { input_path }
    }
}

pub trait FileReader {
    fn read_text(&self) -> Result<Vec<String>, &'static str>;
}

impl FileReader for TextFileReader {
    fn read_text(&self) -> Result<Vec<String>, &'static str> {
        let file_content =
            std::fs::read_to_string(&self.input_path).map_err(|_| "Ошибка чтения файла")?;
        Ok(file_content.lines().map(|s| s.to_string()).collect())
    }
}

pub trait WordFrequency {
    fn build_word_frequency(&self) -> HashMap<String, u32>;
    fn build_sorted_word_frequency(&self) -> Vec<(String, u32)>;
}

impl WordFrequency for TextFileReader {
    fn build_word_frequency(&self) -> HashMap<String, u32> {
        let mut word_frequency = HashMap::new();
        let words = self.read_text().unwrap();
        for word in words {
            *word_frequency.entry(word).or_insert(0) += 1;
        }
        word_frequency
    }

    fn build_sorted_word_frequency(&self) -> Vec<(String, u32)> {
        let mut word_frequency = self.build_word_frequency();
        let mut sorted_word_frequency: Vec<(String, u32)> = word_frequency.drain().collect();
        sorted_word_frequency.sort_by(|a, b| a.0.cmp(&b.0));
        sorted_word_frequency
    }
}

// Задание 4
// Разберите, реализуйте в системе программирования и протестируйте программу,
// соответствующую примеру лекции: Person.
// Предусмотрите создание дополнительного свойства и метода,
// а также трех экземпляров класса в массиве.
// Значения полей выберите самостоятельно.
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String, // Новое свойство
}

impl Person {
    pub fn new(name: String, age: u32, email: String) -> Self {
        Person { name, age, email }
    }

    pub fn print(&self) {
        println!(
            "Имя: {}, Возраст: {}, Email: {}",
            self.name, self.age, self.email
        );
    }

    // Метод для обновления электронной почты
    pub fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: "Никита".to_string(),
            age: 25,
            email: "nikita@example.com".to_string(),
        }
    }
}

// Задание 5
// Проработать примеры
// https://codelessons.dev/ru/klassy-v-c-rukovodstvo-dlya-nachinayushhix/
// до «Конструктор и деструктор класса» включительно.
#[derive(Debug)]
pub enum LinkedList<T> {
    Empty,
    Node(T, Box<LinkedList<T>>),
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList::Empty
    }

    pub fn prepend(self, elem: T) -> Self {
        LinkedList::Node(elem, Box::new(self))
    }

    pub fn len(&self) -> usize {
        match *self {
            LinkedList::Empty => 0,
            LinkedList::Node(_, ref next) => 1 + next.len(),
        }
    }

    pub fn print(&self) {
        match *self {
            LinkedList::Empty => println!("Конец списка"),
            LinkedList::Node(ref data, ref next) => {
                println!("Значение: {:?}", data);
                next.print();
            }
        }
    }
}

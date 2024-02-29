use crate::exercises::{
    bubble_sort, read_single_value, selection_sort, FileReader, LinkedList, Person, TextFileReader,
    WordFrequency,
};

mod exercises;

fn main() {
    // Задание 1
    let mut numbers = Vec::new();
    println!("Введите натуральное число (или 0 для завершения):");
    loop {
        let number: u32 = read_single_value().unwrap();
        if number == 0 {
            break;
        }
        numbers.push(number);
    }
    numbers.sort();
    println!("Введенные числа в порядке возрастания: {:?}", numbers);

    // Задание 2
    let mut array = vec![5, 3, 2, 1, 4];
    bubble_sort(&mut array);
    println!("Сортировка пузырьком: {:?}", array);

    let mut array = vec![53, 23, 2, 1, 4, 0, 2, 1, 4, 0, 5, 3, 2, 1, 4, 0];
    selection_sort(&mut array);
    println!("Сортировка выбором: {:?}", array);

    // Задание 3
    let file_with_words = TextFileReader::new("words.txt".parse().unwrap());
    let sorted_word_frequency = file_with_words.build_sorted_word_frequency();
    println!(
        "Сортированный алфавитно-частотный словарь: {:?}",
        sorted_word_frequency
    );

    // Задание 4
    let mut people = Vec::new();

    // Создание экземпляров Person
    people.push(Person::new(
        "Алексей".to_string(),
        30,
        "alexey@example.com".to_string(),
    ));

    people.push(Person::new(
        "Мария".to_string(),
        28,
        "maria@example.com".to_string(),
    ));

    people.push(Person::default());

    // Вывод информации о каждом человеке
    for person in &people {
        person.print();
    }

    // Обновление email для первого человека в списке
    people[0].update_email("new_alexey@example.com".to_string());
    println!("После обновления email:");
    people[0].print();

    // Задание 5
    let list = LinkedList::new().prepend(1).prepend(2).prepend(3);
    println!("Длина связанного списка: {}", list.len());
    list.print();


    /*
    let mut x = 1; // Исходное значение переменной
    let x_ptr1 = &mut x as *mut i32; // Первый мутабельный указатель на x
    let x_ptr2 = &mut x as *mut i32; // Второй мутабельный указатель на x

    let c = &mut x;
    let cc = &mut x;
    *c += 2;
    *cc += 3;

    unsafe {
        *x_ptr1 += 2;
        *x_ptr2 += 3;
        println!("x = {}", x);
    }
     */

}


use std::cmp::Ordering::Equal;
use std::fmt::Display;
use std::marker::PhantomData;

// Определение структуры узла списка
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Определение структуры односвязного списка
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    //_marker: PhantomData<T>,
}

impl<T> LinkedList<T> {
    // Создание нового пустого списка
    fn new() -> Self {
        LinkedList {
            head: None,
            //_marker: PhantomData,
        }
    }

    // Добавление элемента в начало списка
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Получение максимального элемента списка
    fn max_element(&self) -> Option<&T>
        where
            T: PartialOrd,
    {
        self.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
    }

    // Создание нового списка, содержащего порядковые номера максимальных элементов текущего списка
    fn max_indices(&self) -> LinkedList<usize>
        where
            T: PartialOrd + PartialEq,
    {
        let max_value = self.max_element();
        max_value.map_or_else(
            || LinkedList::new(),
            |max_value| {
                self.iter()
                    .enumerate()
                    .fold(LinkedList::new(), |mut acc, (index, data)| {
                        if data == max_value {
                            acc.push(index);
                        }
                        acc
                    })
            },
        )
    }

    // Итератор по элементам списка
    fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }
}

// Структура итератора по элементам списка
struct LinkedListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}

// Реализация отображения списка для вывода
impl<T> Display for LinkedList<T>
    where
        T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.iter().fold(String::new(), |mut acc, data| {
            acc.push_str(&data.to_string());
            acc.push(' ');
            acc
        });
        write!(f, "{}", result.trim())
    }
}

fn main() {
    // Создание списка List1
    let mut list1 = LinkedList::new();
    list1.push(5.5);
    list1.push(10.2);
    list1.push(7.1);
    list1.push(10.0);
    list1.push(3.5);

    println!("List1: {}", list1);

    // Создание списка List2, содержащего порядковые номера максимальных элементов списка List1
    let list2 = list1.max_indices();

    println!("List2: {}", list2);
}
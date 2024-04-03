use std::fmt;

// Определение структуры узла списка
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

// Определение структуры односвязного списка
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Создание нового пустого списка
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Добавление элемента в начало списка
    fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Получение максимального элемента списка
    // self.iter().fold() используется для поиска максимального элемента.
    // Начальное значение None используется для обработки пустого списка.
    // На каждой итерации, если текущий максимум равен None, ему присваивается текущий элемент.
    // В противном случае, сравнивается текущий элемент с максимумом и возвращается новый максимум.
    fn max_element(&self) -> Option<i32> {
        self.iter().fold(None, |max, &data| {
            max.map_or(Some(data), |m| Some(if data > m { data } else { m }))
        })
    }

    // Создание нового списка, содержащего порядковые номера максимальных элементов текущего списка
    // использует self.iter().enumerate().fold() для создания нового списка с порядковыми номерами максимальных элементов.
    // Метод enumerate() используется для получения пар (индекс, значение) при итерации по списку.
    fn max_indices(&self) -> LinkedList {
        let max_value = self.max_element();
        max_value.map_or_else(
            || LinkedList::new(),
            |max_value| {
                self.iter()
                    .enumerate()
                    .fold(LinkedList::new(), |mut acc, (index, &data)| {
                        if data == max_value {
                            acc.push(index as i32);
                        }
                        acc
                    })
            },
        )
    }

    // Итератор по элементам списка
    fn iter(&self) -> LinkedListIter {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }
}

// Структура итератора по элементам списка
struct LinkedListIter<'a> {
    current: Option<&'a Node>,
}

// Реализация итератора по элементам списка
impl<'a> Iterator for LinkedListIter<'a> {
    type Item = &'a i32;

    // Возвращает следующий элемент списка
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}

// Реализация отображения списка для вывода
impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = self.iter().fold(String::new(), |mut acc, &data| {
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
    list1.push(5);
    list1.push(10);
    list1.push(7);
    list1.push(10);
    list1.push(3);

    println!("List1: {}", list1);

    // Создание списка List2, содержащего порядковые номера максимальных элементов списка List1
    let list2 = list1.max_indices();

    println!("List2: {}", list2);
}

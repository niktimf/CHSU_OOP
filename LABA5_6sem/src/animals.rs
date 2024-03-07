// Задание 1
// Постройте класс TPet (домашнее животное) с двумя скрытыми полями: FName (имя) и FAge (возраст).
// Они должны быть доступны для чтения через свойства name и age и недоступны для записи.
// Метод gettingOlder увеличивает возраст на 1 год. Класс TPet – абстрактный, он имеет абстрактный метод say.
// Постройте два класса-наследника – TCat (кошка) и TDog (собака).Они должны реализовать метод say.
// Описания классов должны быть в отдельном модуле animals
pub trait Pet {
    fn name(&self) -> &str;
    fn age(&self) -> u32;
    fn getting_older(&mut self);
    fn say(&self) -> String;
}

#[derive(Debug)]
pub struct Cat {
    name: String,
    age: u32,
}

impl Cat {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

impl Pet for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn getting_older(&mut self) {
        self.age += 1;
    }

    fn say(&self) -> String {
        String::from("Мяу!")
    }
}

#[derive(Debug)]
pub struct Dog {
    name: String,
    age: u32,
}

impl Dog {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

impl Pet for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn getting_older(&mut self) {
        self.age += 1;
    }

    fn say(&self) -> String {
        String::from("Гав!")
    }
}

// Задание 2
// Добавьте класс TMammal (млекопитающее) – наследник класса TPet и предок для классов TCat и TDog.
// Он должен иметь метод run (бежать), который выводит сообщение вида «Вася побежал».
pub trait Mammal: Pet {
    fn run(&self) {
        println!("{} побежал...", self.name());
    }
}

impl Mammal for Cat {}

impl Mammal for Dog {}

// Задание 3
// Добавьте класс TReptilia (рептилии) – наследник класса TPet и предок для новых классов TTurtle (черепаха) и TSnake (змея).
// Он должен иметь метод crawl (ползти), который выводит сообщение вида «Вася пополз…».
pub trait Reptile: Pet {
    fn crawl(&self) {
        println!("{} ползет...", self.name());
    }
}

pub struct Turtle {
    name: String,
    age: u32,
}

impl Turtle {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

impl Pet for Turtle {
    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn getting_older(&mut self) {
        self.age += 1;
    }

    fn say(&self) -> String {
        String::from("Загадочно молчит...")
    }
}

impl Reptile for Turtle {}

pub struct Snake {
    name: String,
    age: u32,
}

impl Snake {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

impl Pet for Snake {
    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn getting_older(&mut self) {
        self.age += 1;
    }

    fn say(&self) -> String {
        String::from("Шипит...")
    }
}

impl Reptile for Snake {}

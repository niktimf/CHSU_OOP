use crate::animals::{Cat, Dog, Mammal, Pet, Reptile, Snake, Turtle};

mod animals;

fn main() {
    // Задание 1
    let mut pets: Vec<Box<dyn Pet>> = Vec::new();

    let mut dog = Dog::new("Шарик", 5);

    println!("Молодой {}: {} лет", dog.name(), dog.age());
    dog.getting_older();
    println!("Старый {}: {} лет", dog.name(), dog.age());

    let mut cat = Cat::new("Мурка", 3);

    println!("Молодая {}: {} лет", cat.name(), cat.age());
    cat.getting_older();
    println!("Старая {}: {} лет", cat.name(), cat.age());

    pets.push(Box::new(dog));
    pets.push(Box::new(cat));

    pets.iter().for_each(|p| {
        println!("{} говорит {}", p.name(), p.say());
    });

    // Задание 2
    let mut mammals: Vec<Box<dyn Mammal>> = Vec::new();
    let dog = Dog::new("Шарик", 5);
    let cat = Cat::new("Мурзик", 3);
    mammals.push(Box::new(dog));
    mammals.push(Box::new(cat));

    mammals.iter().for_each(|m| {
        m.run();
    });

    // Задание 3
    let mut reptiles: Vec<Box<dyn Reptile>> = Vec::new();
    let turtle = Turtle::new("Торти", 100);
    let snake = Snake::new("Аспид", 10);
    reptiles.push(Box::new(turtle));
    reptiles.push(Box::new(snake));

    reptiles.iter().for_each(|r| {
        r.crawl();
        println!("{} {}", r.name(), r.say());
    });
}

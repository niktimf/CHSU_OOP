use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::str::FromStr;
use phonenumber::PhoneNumber;


struct CreditCard(String);

impl fmt::Display for CreditCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for CreditCard {
    fn from(value: T) -> Self {
        CreditCard(value.into())
    }
}

struct Customer {
    surname: String,
    name: String,
    patronymic: String,
    address: String,
    phone: PhoneNumber,
    credit_card_number: CreditCard,
    bank_account_number: u64,
}

impl Customer {
    fn new(
        surname: String,
        name: String,
        patronymic: String,
        address: String,
        phone: PhoneNumber,
        credit_card_number: CreditCard,
        bank_account_number: u64,
    ) -> Self {
        Customer {
            surname,
            name,
            patronymic,
            address,
            phone,
            credit_card_number,
            bank_account_number,
        }
    }

    fn show(&self) {
        println!("Surname: {}", self.surname);
        println!("Name: {}", self.name);
        println!("Patronymic: {}", self.patronymic);
        println!("Address: {}", self.address);
        println!("Phone: {}", self.phone);
        println!("Credit Card Number: {}", self.credit_card_number);
        println!("Bank Account Number: {}", self.bank_account_number);
    }
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.surname, self.name, self.patronymic
        )
    }
}


impl PartialEq<Self> for CreditCard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for CreditCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

fn main() {
    let mut customers = vec![
        Customer::new(
            "Иванов".to_string(),
            "Иван".to_string(),
            "Иванович".to_string(),
            "ул. Ленина, 10".to_string(),
            PhoneNumber::from_str("+71234567890").unwrap(),
            CreditCard::from("1234_5678_9000_0000"),
            1234567890,
        ),
        Customer::new(
            "Петров".to_string(),
            "Петр".to_string(),
            "Петрович".to_string(),
            "ул. Мира, 20".to_string(),
            PhoneNumber::from_str("+79876543210").unwrap(),
            CreditCard::from("9876_5432_1000_0000"),
            987654321,
        ),
        Customer::new(
            "Сидоров".to_string(),
            "Сидор".to_string(),
            "Сидорович".to_string(),
            "ул. Пушкина, 30".to_string(),
            PhoneNumber::from_str("+71112223344").unwrap(),
            CreditCard::from("1111_2222_3300_0000"),
            1111_2222_33,
        ),
    ];

    // а) список покупателей в алфавитном порядке
    customers.sort_by(|a, b| a.surname.cmp(&b.surname));
    println!("Список покупателей в алфавитном порядке:");
    for customer in &customers {
        println!("{}", customer);
    }

    // б) список покупателей, номер кредитной карточки которых находится в заданном интервале
    let min_credit_card = CreditCard::from("1000_0000_0000_0000");
    let max_credit_card = CreditCard::from("2000_0000_0000_0000");
    println!("\nСписок покупателей с номером кредитной карточки в интервале от {} до {}:", min_credit_card, max_credit_card);
    for customer in &customers {
        if customer.credit_card_number >= min_credit_card && customer.credit_card_number <= max_credit_card {
            println!("{}", customer);
        }
    }
}
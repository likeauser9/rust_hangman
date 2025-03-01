use rand::prelude::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
struct Person {
    name: String,
    age: i32,
}

#[allow(dead_code)]
enum Options {
    Add,
    Remove,
}

fn main() {
    let mut words: Vec<&str> = vec!["Weather", "Window", "Spring", "Scooter"];
    let mut user_option: String = String::new();
    println!("Виселица\n");
    println!("Введите 'start', чтобы начать игру или 'exit', чтобы завершить игру");
    loop {
        print!("Введите вариант: ");
        io::stdout().flush().unwrap();
        user_option.clear();

        if io::stdin().read_line(&mut user_option).is_err() {
            println!("Ошибка ввода. Попробуйте снова.");
            continue;
        }

        match user_option.trim() {
            "start" => {
                println!("Игра начинается...");
            }
            "exit" => {
                println!("Выход из игры");
                thread::sleep(Duration::from_secs(2));
                break;
            }
            _ => {
                println!("Неверный ввод, попробуйте снова.");
                continue;
            }
        }

        let mut rng: ThreadRng = rand::rng();
        words.shuffle(&mut rng);
        let word: String = words.choose(&mut rng).unwrap().to_lowercase();
        let word_chars: Vec<char> = word.chars().collect();
        let mut guessed_word: Vec<char> = vec!['*'; word.len()];

        let mut attempts = 6;
        let mut guessed_letters: Vec<char> = vec![];

        while attempts > 0 && guessed_word.contains(&'*') {
            println!("Слово {}", guessed_word.iter().collect::<String>());
            println!("Оставшиеся попытки: {}", attempts);
            println!("Угаданные буквы {:?}", guessed_letters);

            print!("Введите букву: ");
            io::stdout().flush().unwrap();
            let mut user_letter: String = String::new();
            io::stdin().read_line(&mut user_letter).unwrap();

            let user_letter: String = user_letter.trim().to_lowercase();
            if user_letter.len() != 1 {
                println!("Введите только 1 букву!");
                continue;
            }

            let letter = user_letter.chars().next().unwrap();
            if guessed_letters.contains(&letter) {
                println!("Вы уже вводили эту букву!");
                continue;
            }

            guessed_letters.push(letter);

            if word_chars.contains(&letter) {
                for (i, &c) in word_chars.iter().enumerate() {
                    if c == letter {
                        guessed_word[i] = letter;
                    }
                }
                println!("Верно!");
            } else {
                attempts -= 1;
                println!("Неверно! Попыток осталось: {}", attempts);
            }
        }

        if guessed_word.contains(&'*') {
            println!("\nВы проиграли! Загаданное слово было: {}", word);
        } else {
            println!("\nВы отгадали слово: {}", word);
        }

        print!("Хотите поиграть снова? (да/нет): ");
        io::stdout().flush().unwrap();
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).unwrap();
        if play_again.trim().to_lowercase() != "да" {
            println!("Спасибо за игру!");
            break;
        }
    }
}

// rand::seq::slice::IndexedRandom
// pub trait IndexedRandom
// pub fn choose<R>(&self, rng: &mut R) -> Option<&Self::Output>
// where
//     R: Rng + ?Sized,
//     // Bounds from trait:
//     Self: Index<usize>,

// core::slice
// impl<T> [T]
// pub fn contains(&self, x: &T) -> bool
// where
//     T: PartialEq,

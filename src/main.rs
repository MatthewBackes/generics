use std::vec;
use summary::{NewsArticle, Summary, Tweet};

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2).sqrt()
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn notify(item: &impl Summary) {
    println!("Breaking! {}", item.summarize());
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point {x: 5, y: 10};
    let float = Point {x:1.0, y: 4.0};
    // let err = Point {x: 3, y: 5.0}; Error: generic T is set to integer when x is assigned 3. Trying to assign float at y causes an error.

    let tweet = Tweet{
        username: String::from("testuser"),
        content: String::from("this is a test tweet"),
        reply: false,
        retweet: false,
    };

    println!("New tweet: {}", tweet.summarize());
    println!("Author: {}", tweet.readmore());

    let article = NewsArticle {
        headline: String::from("Test Article"),
        location: String::from("Test Location"),
        author: String::from("Test Author"),
        content: String::from("Test Body Text."),
    };
    println!("New arictle {}", article.summarize());

    notify(&tweet);
}
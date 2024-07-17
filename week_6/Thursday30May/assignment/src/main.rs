// Create a struct that has one type and uses generics, have one implementation of the struct then print out the contents of the struct
use std::fmt::Debug;
#[derive(Debug)]
struct Video<T> {
    views: T,
    comments: T,
    likes: T
}

impl<T: Debug> Video<T> {
    fn new(views: T, comments: T, likes: T) -> Self {
        Video{views, comments, likes}
    }

    fn print_all(&self) {
        println!("This video has {:?} views, {:?} comments and {:?} likes.", self.views, self.comments, self.likes);
    }

    fn print_views(&self) -> &T {
        &self.views
    }

    fn print_comments(&self) -> &T {
        &self.comments
    }

    fn print_likes(&self) -> &T {
        &self.likes
    }
}

fn main() {
    let tiktok = Video::new(1000, 350, 749);
    tiktok.print_all();
    println!("This video has ganered views that total to: {}", tiktok.print_views());
    println!("This video has ganered comments that total to: {}", tiktok.print_comments());
    println!("This video has ganered likes that total to: {}", tiktok.print_likes());
}

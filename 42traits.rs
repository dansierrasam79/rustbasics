pub trait Items
{
    fn display_information(&self) -> String;
    fn available_or_not(&self) -> bool;
}
 
struct Book
 {
  book_name:String,
  author_name:String,
  pages: i32,
  available: bool
}

 
// Implementing a trait Items on the Book struct
impl Items for Book  {
 // Method returns book details
  fn display_information(&self) -> String{
  return format!("{} was written by {} consisting of {} pages",self.book_name, 
                                    self.author_name, self.pages.to_string());
  }
   
  // Method returns whether book is available or not
  fn available_or_not(&self) -> bool{
  return self.available
  }
}
 
fn main()
{
  let book = Book{
  book_name: "World War Z".to_string(),
  author_name: "Max Brooks".to_string(),
  pages: 342,
  available: true,
  };
   
  println!("{}", book.display_information());
  println!("Book Available? {}", book.available_or_not());
}
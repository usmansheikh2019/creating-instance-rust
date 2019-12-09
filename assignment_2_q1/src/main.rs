#[derive(Debug)]
struct Book{
    author:String,
    name:String
}
impl Book{
    fn book_details(author:String,name:String)->Book{
         Book{
             author,
             name,
         }
    }
    
}
trait BookInformation{
    fn info(&self)->String;
}
impl BookInformation for Book{
    fn info(&self)->String{
        
            format!("Author : {}  Name : {}",self.author,self.name)
            
        }
    }

fn main()
{
    let book_01 = Book::book_details(String::from("George R. R. Martin"),String::from("Game Of Thrones"));
    println!("{:?}",book_01.info());
}

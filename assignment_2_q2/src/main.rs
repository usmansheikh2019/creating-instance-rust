#[derive(Debug)]
struct Student{
    name:String
}
// This Is How We Make An Instance By Associative Function 
// impl Student{
//     fn create_student(name:String)->Student{
//         Student{
//             name,
//         }
//     }
//     fn display_student(&self)->String{
//         format!("{}",self.name)
//     }
// }
fn main()
{   
    // This Is How We Make Instance Directly In main()
    let student_01 = Student{name:"Zain".to_string()};
    println!("{:?}",student_01); 
}
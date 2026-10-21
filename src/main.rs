#[derive(Clone, Debug)]
enum LinkedList{
    Nil,
    Node(i32, Box<LinkedList>),
}

impl LinkedList{
    #[allow(dead_code)]
    fn display_list(&self) {
        match self{
            LinkedList::Nil => println!("nil"),
            LinkedList::Node(value, tail) => {
                println!("node {}", value);
                tail.display_list();
            }
        }
    }

    fn push_back(&mut self, value: i32){
        match self{
            LinkedList::Node(_, tail) => tail.push_back(value),
            LinkedList::Nil => *self = LinkedList::Node(value, Box::new(LinkedList::Nil)),
        }
    }

    fn push_front(&mut self, value: i32){
        *self = LinkedList::Node(value, Box::new(self.clone()));
    }

    fn delete(&mut self, index: usize) -> Option<i32>{
        if index == 0 {
            match self{
                LinkedList::Nil => None,
                LinkedList::Node(value, tail) => {
                    let temp = value.clone();
                    *self = *tail.clone();
                    Some(temp)
                }
            }
        } else {
            match self{
                LinkedList::Nil => None,
                LinkedList::Node(_value, tail) => {
                    tail.delete(index - 1)
                }
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::Nil;
    list.push_back(10);
    list.push_back(20);
    //list.display_list();
    println!("Realizando o push_front...");
    list.push_front(30);
    println!("push_front concluído!");
    list.display_list();
    println!("Realizando o delete...");
     list.delete(0);
    println!("delete concluído!");
    //list.display_list();
}

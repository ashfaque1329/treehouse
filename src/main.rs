use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action:VisitorAction,
    age:i8
}

#[derive(Debug)]
enum VisitorAction{
    Accept,
    AcceptWithNote {note:String},
    Refuse,
    Probation
}

impl Visitor {
    fn new(name: String, action:VisitorAction, age:i8) -> Self {
        Self {
            name: name.to_lowercase().to_string(),
            action,
            age
        }
    }

    fn greet_vistor(&self) {
        match &self.action{
            VisitorAction::Accept=>println!("Welcome to the treehouse, {}",self.name),
            VisitorAction::AcceptWithNote {note}=>{
                println!("Welcome to the treehouse, {}",self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}",self.name);
                }
            }
            VisitorAction::Probation=>println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse=>println!("Do not allow {} in!", self.name),
        }
    }

}

fn what_is_your_name() -> String {
    let mut name: String = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    loop{
        println!("Hello, what's your name?");
        let name: String = what_is_your_name();

        let mut visitor_list:Vec<Visitor> = vec![Visitor::new(String::from("John"),VisitorAction::Accept,36),
        Visitor::new(String::from("Gabriel"),VisitorAction::AcceptWithNote {note:String::from("Lactose-free milk is in the refrigerator")},15),Visitor::new(String::from("Mikael"),VisitorAction::Refuse,30)];

        /*

        let mut allow_them_in: bool = false;

        for visitor in visitor_list {
            if visitor.name == name {
                allow_them_in = true;
            }
        }

         if allow_them_in {
            println!("Welcome to the Treehouse, {}", name);
        } else {
            println!("Sorry, you aren't on the list.");
        }
        */

        let known_visitor=visitor_list.iter().find(|visitor|visitor.name==name);

        match known_visitor{
            Some(visitor)=>visitor.greet_vistor(),
            None=>{
                if name.is_empty(){
                    break;
                } else {
                    visitor_list.push(Visitor::new(name,VisitorAction::Probation,0));
                    visitor_list[visitor_list.len()-1].greet_vistor();
                }

            }
        }
        //break;
        println!("Visitor list: {:#?}",visitor_list);
    }



}

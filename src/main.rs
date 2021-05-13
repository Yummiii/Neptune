mod arguments;

use arguments::Options;

fn main() {
    let args = Options::build();
    
    if args.password {
        println!("Senha");
    } else if args.timestamp {
        println!("Timestamp");
    } else if args.uuid {
        println!("UUID");
    }
}
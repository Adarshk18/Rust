use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)] //list of attributes  ...trait is a collection of functions

struct Deck { //strcut is used to define collection of fields
    cards: Vec<String>,
}

impl Deck{
    fn new() -> Self{
        //list of 'suits'-'hearts','spades'
    let suits= vec!["Hearts","Spades","Diamonds"];
    let values = vec!["Ace","Two","Three"];

    let mut cards = vec![];

    for suit in suits{
        for value in &values{
            let card = format!("{} of {}",value,suit);
            cards.push(card);
        }
    }


    //a variable is a bindings
    Deck {cards}
    
    }

    fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize)->Vec<String>{
        self.cards.split_off(
            self.cards.len()-num_cards
        )
    }
}

fn main() {
    
let mut deck = Deck::new();
let cards = deck.deal(3);
    println!("Here's your hand: {:#?}",cards);
    println!("Here's your deck: {:#?}",deck);
}

use rand::{rng,seq::SliceRandom};
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck{
    fn new()-> Self{
    let suits=["Hearts","Spades","Diamonds"]; 
    let values=["Ace","two","three"];

    let mut cards=vec![];
    for suit in suits{
        for value in values{
            let card=format!("{} of {}",value,suit);
            cards.push(card);
        }
    }
    let deck = Deck { cards: cards };
    return deck;
    } 

    fn shuffle(&mut self){
        let mut rng=rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String>{
       return self.cards.split_off(self.cards.len() - num_cards);
    }
} 
fn main() {
    let mut deck=Deck::new();
    deck.shuffle();
    println!("Here is your deck {:#?}",deck);
    let cards=deck.deal(3);
    println!("Here is your hand {:#?}",cards);
}

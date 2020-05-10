//extern crate rand;
use rand::Rng;

#[derive(Clone)]
struct Card {
	rank: u8,
	suit: u8,
}

impl Card {
	fn new(rank: u8, suit: u8) -> Card {
		Card {
			rank: rank,
			suit: suit,
		}
	}
}

#[derive(Clone)]
struct Casino {
	deck: Vec<Card>,
	discard: Vec<Card>,
	hands: Vec<Vec<Card>>
}

impl Casino {
	fn new() -> Casino {
		Casino {
			deck: Casino::clean(),
			discard: vec![],
			hands: vec![]
		}
	}

	fn clean() -> Vec<Card> {
		let mut unfinished = vec![];

		for suit in 0..4 {
			for rank in 0..13 {
				unfinished.push(Card {
					suit: suit as u8,
					rank: rank as u8,
				});
			}
		}

		unfinished
	}

	fn card_string(c: &Card) -> String {
		let ranks = [
			"Two",
			"Three",
			"Four",
			"Five",
			"Six",
			"Seven",
			"Eight",
			"Nine",
			"Ten",
			"Jack",
			"Queen",
			"King",
			"Ace",
		];

		let suits = [
			"Hearts",
			"Diamonds",
			"Clubs",
			"Spades",
		];

		format!("{} of {}", ranks[c.rank as usize], suits[c.suit as usize])
	}

	fn deal(deck: &mut Vec<Card>, hands: &mut Vec<Vec<Card>>, count: u32, hand_count: u8) {
		for _repeat in 0..hand_count {
			let mut h = vec![];

			if deck.len() >= count as usize {
				for i in 0..count as usize {
					h.push(deck[i].clone());
				}

				deck.drain(0..count as usize);
				hands.push(h);
			 }
		}
	}

	fn deck_strings(deck: Vec<Card>) -> Vec<String> {
		let mut veck = vec![];

		for i in 0..deck.len() {
			veck.push(Casino::card_string(&deck[i]));
		}

		veck
	}

	//merge sort for a vec of cards
	fn sort(d: &mut Vec<Card>) {
		if d.len() > 1 {
			let mut chunk_size = d.len() / 2;
			if d.len() % 2 != 0 {
				chunk_size += 1;
			}

			let mut chunks = vec![];
			for chunk in d.chunks(chunk_size) {
				chunks.push(chunk.to_vec());
			}

			for i in 0..chunks.len() {
				Casino::sort(&mut chunks[i]);
			}

			let mut sorted = vec![];
			let mut a = 0;
			let mut b = 0;
			while a < chunks[0].len() && b < chunks[1].len() {
				let avar = chunks[0][a].rank;
				let bvar = chunks[1][b].rank;
				if avar <= bvar {
					sorted.push(chunks[0][a].clone());
					a += 1;
				}else{
					sorted.push(chunks[1][b].clone());
					b += 1;
				}
			}

			for i in a..chunks[0].len() {
				sorted.push(chunks[0][i].clone());
			}

			for i in b..chunks[1].len() {
				sorted.push(chunks[1][i].clone());
			}

			*d = sorted;
		}
	}
	
	//merge sort for a 2d u8 vector
	fn sort2d(d: &mut Vec<Vec<u8>>) {
		if d.len() > 1 {
			let mut chunk_size = d.len() / 2;
			if d.len() % 2 != 0 {
				chunk_size += 1;
			}

			let mut chunks = vec![];
			for chunk in d.chunks(chunk_size) {
				chunks.push(chunk.to_vec());
			}

			for i in 0..chunks.len() {
				Casino::sort2d(&mut chunks[i]);
			}

			let mut sorted = vec![];
			let mut a = 0;
			let mut b = 0;
			while a < chunks[0].len() && b < chunks[1].len() {
				let avar = chunks[0][a][0];
				let bvar = chunks[1][b][0];
				if avar <= bvar {
					sorted.push(chunks[0][a].clone());
					a += 1;
				}else{
					sorted.push(chunks[1][b].clone());
					b += 1;
				}
			}

			for i in a..chunks[0].len() {
				sorted.push(chunks[0][i].clone());
			}

			for i in b..chunks[1].len() {
				sorted.push(chunks[1][i].clone());
			}

			*d = sorted;
		}
	}
	
	fn shuffle(d: &mut Vec<Card>) {
		let mut rng = rand::thread_rng();

		let mut shuf = vec![];
		let mut used = vec![d.len()];

		let n = d.len() - 1;
		let shuffle_count = n * n;
		println!("shuffle_count: {}", shuffle_count);

		for _reshuffle in 0..shuffle_count {
			for _i in 0..d.len() {
				let mut pos = rng.gen_range(0,d.len());
				
				while used.iter().any(|x| x == &pos) {
					pos += 1;
					if pos >= d.len() {
						pos = 0;
					}
				}

				shuf.push(d[pos].clone());
				used.push(pos);
			}

			*d = shuf.clone();
			shuf = vec![];
			used = vec![d.len()];
		}
	}
}

fn straight(h: &Vec<Card>) -> Vec<u8> {
	//get int vec of ranks
	let mut ranks = vec![];
	for c in h.iter() {
		ranks.push(c.rank);
	}

	//find lowest value
	let mut low = ranks[0];
	for i in 1..h.len() {
		if ranks[i] < low {
			low  = ranks[i];
		}
	}

	//convert ace if the low is 0
	if low == 0 {
		for i in 0..ranks.len() {
			if ranks[i] == 12 {
				ranks[i] = 0 + ranks.len() as u8;
			}
		}
	}

	//check if it is a straight
	let mut good = true;
	for i in low..low + 5 {
		if !ranks.iter().any(|x| *x == i) {
			good = false;
			break;
		}
	}

	//packages the return value of the function
	let mut returner = vec![0];
	
	if good {
		returner = vec![1,low];
	}
	
	returner
}

fn flush(h: &Vec<Card>) -> Vec<u8> {
	let mut r = vec![0];

	if h.iter().all(|x| x.suit == h[0].suit) {
		let mut clone = h.clone();
		Casino::sort(&mut clone);

		let mut ranks = vec![];
		clone.iter().for_each(|x| ranks.push(x.rank));
		let mut reversed = vec![];
		for i in ranks.iter().rev() {
			reversed.push(i.clone());
		}
		ranks = reversed;
		//println!("{:?}", ranks);
		r = ranks;
	}

	r
}

fn sets(h: &Vec<Card>) -> Vec<Vec<u8>> {
	let mut sets = vec![];
	let mut used = vec![13];

	for i in 0..h.len() {
		let rank = h[i].rank;
		if !used.iter().any(|x| *x == rank) {
			let mut count = 0;
			for c in h.iter() {
				if c.rank == rank {
					count += 1;
				}
			}			
			used.push(rank);
			sets.push(vec![rank,count]);
		}
	}	
	Casino::sort2d(&mut sets);
	let mut tmp = vec![];
	for x in sets.iter().rev() {
		tmp.push(x.clone());
	}
	sets = tmp;
	//println!("sets: {:?}", sets);	
	sets
}

fn poker(hands: &mut Vec<Vec<Card>>) -> Vec<usize> {
	let mut rankings = vec![];
	let mut rules: Vec<Box<dyn Fn(&Vec<Card>) -> Vec<u8>>> = vec![];

	//Royal Flush
	rules.push(Box::new(|h: &Vec<Card>| {
		let mut r = vec![0];

		let has_10 = h.iter().any(|x| x.rank == 9);
		let has_ace = h.iter().any(|x| x.rank == 0);
		
		if has_10 && has_ace {
			let s = straight(&h);
			let f = flush(&h);

			if s != vec![0] && f != vec![0] {
				r = vec![1];
			}
		}

		r
	}));
	//Straight Flush
	rules.push(Box::new(|h: &Vec<Card>| {
		let mut r = vec![0];

		let s = straight(&h);
		let f = flush(&h);

		if s.len() > 0 && f != vec![0] {
			r = s;
		}

		r
	}));
	//Four of a Kind
	rules.push(Box::new(|h: &Vec<Card>| {
		let sets = sets(h);
		let mut rtrn = vec![0];
		let mut high = 0;

		for i in 0..sets.len() {
			if sets[i][1] == 4 && sets[i][0] > high{    
				rtrn = vec![1, sets[i][0]];
				high = sets[i][0];
			}
		}

		rtrn
	}));
	//Full House 
	rules.push(Box::new(|h: &Vec<Card>| {
		let sets = sets(h);
		let mut rtrn = vec![0];
		let mut house = vec![];

		for i in (2..4).rev() {
			for x in sets.iter() {
				if x[1] == i {
					house.push(x[0])
				}
			}
		}

		if house.len() == 2 {
			rtrn = vec![1];
			rtrn.extend(house);
		}

		rtrn
	}));
	//Flush
	rules.push(Box::new(|h: &Vec<Card>| {
		flush(&h)
	}));	
	//Straight
	rules.push(Box::new(|h: &Vec<Card>| {
		let s = straight(&h);
		let mut r = vec![0];
		if s.len() > 0 {
			r = s.to_vec();
		}

		r
	}));
	//Three of a Kind
	rules.push(Box::new(|h: &Vec<Card>| {
		let sets = sets(h);
		let mut rtrn = vec![0];
		let mut high = 0;

		for i in 0..sets.len() {
			if sets[i][1] == 3 && sets[i][0] > high{    
				rtrn = vec![1, sets[i][0]];
				high = sets[i][0];
			}
		}

		rtrn
	}));
	//Two Pair
	rules.push(Box::new(|h: &Vec<Card>| {
		let sets = sets(h);
		let mut rtrn = vec![0];
		let mut pairs = vec![];

		for i in 0..sets.len() {
			if sets[i][1] == 2 {    
				pairs.push(sets[i].clone());
			}
		}
		
		if pairs.len() == 2{	
			if pairs[0][0] > pairs[1][0]{
				rtrn = vec![pairs[0][0]];
				rtrn.push(pairs[1][0]);
			}else {
				rtrn = vec![pairs[1][0]];
				rtrn.push(pairs[0][0].clone());
			}
		}

		rtrn
	}));
	//One Pair
	rules.push(Box::new(|h: &Vec<Card>| {
		let sets = sets(h);
		let mut rtrn = vec![0];
		let mut high = 0;

		for i in 0..sets.len() {
			if sets[i][1] == 2 && sets[i][0] > high{    
				rtrn = vec![1, sets[i][0]];
				high = sets[i][0];
			}
		}

		rtrn
	}));
	//High Card
	rules.push(Box::new(|h: &Vec<Card>| {
		let mut high = 0;
		for i in h.iter() {
			let x = i.rank;
			if x > high {
				high = x;
			}
		}
		vec![high]
	}));

	//Rankings
	'hand: for hand_iter in 0..hands.len() {
		let h = &mut hands[hand_iter];
		//println!("Hand: {}", hand_iter);

		for i in 0..rules.len() {
			let out = rules[i](h);
			if out[0] != 0 {
				let mut v = vec![i as u8];
				v.extend(out);
				//println!("{:?}", v);
				rankings.push(v);
				
				continue 'hand;
			}
		}
	}

	//Print Rankings
	for (i, r) in rankings.iter().enumerate() {
		println!("{}: {:?}", i, r);
	}

	//Winner
	let mut best = vec![0];
	for i in 1..rankings.len() {
		let r = &rankings[i];
		let b = best[0];
		let rb = &rankings[b];
		if r[0] < rb[0] as u8 {
			best = vec![i];
		} else if r[0] == rb[0] {
			let mut tie = true;
			for j in 1..r.len() {
				if r[j] > rb[j] {
					best = vec![i];
					tie = false;
					break;
				}
				if r[j] < rb[j] {
					tie = false;
					break;
				}
			}

			if tie {
				best.push(i);
			}
		}
	}

	best
}

fn main() {
	let mut table = Casino::new();
	println!("Casino Opened");

	Casino::shuffle(&mut table.deck);
	println!("Shuffled");
	
	Casino::deal(&mut table.deck, &mut table.hands, 5, 50);
	if true {
		table.hands.push(vec![
			Card::new(12,0),
			Card::new(12,3),
			Card::new(12,1),
			Card::new(8,2),
			Card::new(8,1),
		]);
	}
	println!("Dealt {}", table.hands.len());

	for i in 0..table.hands.len() {
		Casino::sort(&mut table.hands[i]);
		let h = Casino::deck_strings(table.hands[i].clone());
		print!("{}: ", i);
		for i in 0..h.len() {
			print!("{}, ", h[i]);
		}
		println!();
	}

	let winner = poker(&mut table.hands);
	println!("{:?}", winner);
}
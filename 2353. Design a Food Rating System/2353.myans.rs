use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq)]
struct Entry {
    rating: i32,
    name: String,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rating.cmp(&other.rating) {
            Ordering::Equal => other.name.cmp(&self.name),
            ord => ord,
        }
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct FoodRatings {
    // 現在値：food -> (cuisine, rating)
    cur: HashMap<String, (String, i32)>,
    // 各 cuisine ごとの最大ヒープ
    heaps: HashMap<String, BinaryHeap<Entry>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cur = HashMap::new();
        let mut heaps: HashMap<String, BinaryHeap<Entry>> = HashMap::new();

        for ((food, cuisine), &rating) in foods.into_iter().zip(cuisines.iter().cloned()).zip(ratings.iter()) {
            cur.insert(food.clone(), (cuisine.clone(), rating));
            heaps.entry(cuisine)
                .or_default()
                .push(Entry { rating, name: food });
        }

        FoodRatings { cur, heaps }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if !self.cur.contains_key(&food) { return; }

        let cuisine_clone = {
            let (cuisine, rating_ref) = self.cur.get_mut(&food).unwrap();
            *rating_ref = new_rating;      
            cuisine.clone()              
        }; 

        self.heaps
            .entry(cuisine_clone)
            .or_default()
            .push(Entry { rating: new_rating, name: food });
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let heap = self.heaps.get_mut(&cuisine).expect("unknown cuisine");
        loop {
            let top = heap.peek().expect("heap is empty").clone();
            // 現在値と一致するまで捨てる
            if let Some((c, r)) = self.cur.get(&top.name) {
                if *c == cuisine && *r == top.rating {
                    return top.name;
                }
            }
            heap.pop();
        }
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */
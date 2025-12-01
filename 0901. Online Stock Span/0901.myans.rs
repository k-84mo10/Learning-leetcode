struct StockSpanner {
    stocks: Vec<i32>,
    spans: Vec<i32>,
    stack: Vec<usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self {
            stocks: Vec::new(),
            spans: Vec::new(),
            stack: Vec::new(),
        }    
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut current_span = 1;

        while let Some(&day) = self.stack.last() {
            if self.stocks[day] <= price {
                current_span += self.spans[day];
                self.stack.pop();
            } else {
                break;
            }
        }

        let day = self.stocks.len();
        self.stocks.push(price);
        self.spans.push(current_span);
        self.stack.push(day);

        current_span
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
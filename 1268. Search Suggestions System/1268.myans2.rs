impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort();

        let mut result = Vec::new();
        let mut prefix = String::new();

        for ch in search_word.chars() {
            prefix.push(ch);
            
            let idx = products.partition_point(|s| s < &prefix);

            let mut candidate = Vec::new();
            for i in idx..(idx+3).min(products.len()) {
                if products[i].starts_with(&prefix) {
                    candidate.push(products[i].clone());
                } else {
                    break;
                }
            }

            result.push(candidate);
        }

        result
    }
}
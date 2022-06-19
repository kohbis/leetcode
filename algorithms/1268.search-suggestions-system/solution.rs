impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort_unstable();

        let mut res: Vec<Vec<String>> = vec![vec![]; search_word.len()];

        for i in 0..search_word.len() {
            let prefix = &search_word[0..=i];

            for product in &products {
                if res[i].len() < 3 {
                    if product.starts_with(prefix) {
                        res[i].push(product.to_string());
                    }
                } else {
                    break;
                }
            }
        }

        res
    }
}

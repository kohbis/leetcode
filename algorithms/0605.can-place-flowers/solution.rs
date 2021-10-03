impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let len = flowerbed.len();
        let mut flowerbed = flowerbed;
        let mut n = n;

        for i in 0..len {
            if flowerbed[i] == 0 {
                if i == 0 {
                    if len > 1 {
                        if flowerbed[i + 1] == 0 {
                            flowerbed[i] = 1;
                        }
                    } else {
                        flowerbed[i] = 1;
                    }
                } else if i == len - 1 {
                    if flowerbed[i - 1] == 0 {
                        flowerbed[i] = 1;
                    }
                } else {
                    if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
                        flowerbed[i] = 1;
                    }
                }

                if flowerbed[i] == 1 {
                    n -= 1;
                }
            }

            if n < 1 {
                return true;
            }
        }

        false
    }
}

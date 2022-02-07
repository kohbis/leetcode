func canPlaceFlowers(flowerbed []int, n int) bool {
	l := len(flowerbed)

	for i := 0; i < l; i++ {
		if flowerbed[i] == 0 {
			if i == 0 {
				if l > 1 {
					if flowerbed[i+1] == 0 {
						flowerbed[i] = 1
					}
				} else {
					flowerbed[i] = 1
				}
			} else if i == l-1 {
				if flowerbed[i-1] == 0 {
					flowerbed[i] = 1
				}
			} else {
				if flowerbed[i-1] == 0 && flowerbed[i+1] == 0 {
					flowerbed[i] = 1
				}
			}

			if flowerbed[i] == 1 {
				n--
			}
		}

		if n < 1 {
			return true
		}
	}

	return false
}

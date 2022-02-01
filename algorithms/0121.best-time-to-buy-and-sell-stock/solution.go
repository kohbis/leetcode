func maxProfit(prices []int) int {
	maxProfit := 0
	buyPrice := prices[0]

	for _, price := range prices {
		if price < buyPrice {
			buyPrice = price
		}

		if maxProfit < price-buyPrice {
			maxProfit = price - buyPrice
		}
	}

	return maxProfit
}

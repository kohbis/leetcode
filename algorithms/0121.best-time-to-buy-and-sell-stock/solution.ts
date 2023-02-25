function maxProfit(prices: number[]): number {
  let max = 0;
  let buyPrice = prices[0];

  for (const price of prices) {
    if (price < buyPrice) {
      buyPrice = price;
    }

    if (max < price - buyPrice) {
      max = price - buyPrice;
    }
  }

  return max;
}

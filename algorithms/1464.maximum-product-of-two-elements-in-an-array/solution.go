func maxProduct(nums []int) int {
	max, second_max := 0, 0
	for _, num := range nums {
		if num > max {
			second_max = max
			max = num
		} else if num > second_max {
			second_max = num
		}
	}
	return (max - 1) * (second_max - 1)
}

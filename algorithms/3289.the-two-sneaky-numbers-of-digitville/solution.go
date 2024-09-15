func getSneakyNumbers(nums []int) []int {
	sneakyNumbers := []int{}
	count := make(map[int]int)
	for _, num := range nums {
		count[num]++
		if count[num] > 1 {
			sneakyNumbers = append(sneakyNumbers, num)
		}
		if len(sneakyNumbers) == 2 {
			break
		}
	}
	return sneakyNumbers
}

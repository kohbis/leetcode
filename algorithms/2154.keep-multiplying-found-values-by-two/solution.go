func findFinalValue(nums []int, original int) int {
	target := original
	for {
		if contains(nums, target) {
			target *= 2
		} else {
			break
		}
	}

	return target
}

func contains(arr []int, x int) bool {
	for _, v := range arr {
		if v == x {
			return true
		}
	}
	return false
}


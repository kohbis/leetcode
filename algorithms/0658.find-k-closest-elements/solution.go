
func findClosestElements(arr []int, k int, x int) []int {
	left := 0
	right := len(arr) - 1

	for right-left+1 > k {
		if arr[right]-x < x-arr[left] {
			left++
		} else {
			right--
		}
	}

	return arr[left:(left + k)]
}

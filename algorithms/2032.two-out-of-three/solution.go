func twoOutOfThree(nums1 []int, nums2 []int, nums3 []int) []int {
	res := []int{}

	for i := 1; i <= 100; i++ {
		count := 0

		if contains(nums1, i) {
			count++
		}

		if contains(nums2, i) {
			count++
		}

		if contains(nums3, i) {
			count++
		}

		if count >= 2 {
			res = append(res, i)
		}
	}

	return res
}

func contains(arr []int, x int) bool {
	for _, v := range arr {
		if v == x {
			return true
		}
	}
	return false
}

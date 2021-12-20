func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	nums := merge(nums1, nums2)
	mid := len(nums) / 2

	if len(nums)%2 == 0 {
		return float64((nums[mid-1] + nums[mid])) / 2.0
	} else {
		return float64(nums[mid])
	}
}

func merge(s []int, t []int) []int {
	merged := []int{}
	si, ti := 0, 0

	for si < len(s) && ti < len(t) {
		if s[si] < t[ti] {
			merged = append(merged, s[si])
			si++
		} else {
			merged = append(merged, t[ti])
			ti++
		}
	}

	for si < len(s) {
		merged = append(merged, s[si])
		si++
	}

	for ti < len(t) {
		merged = append(merged, t[ti])
		ti++
	}

	return merged
}

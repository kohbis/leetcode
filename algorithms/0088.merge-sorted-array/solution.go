func merge(nums1 []int, m int, nums2 []int, n int) {
	nums1 = nums1[:m]
	for _, num := range nums2 {
		nums1 = append(nums1, num)
	}
	sort.Ints(nums1)
}

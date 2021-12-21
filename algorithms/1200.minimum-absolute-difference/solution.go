func minimumAbsDifference(arr []int) [][]int {
	res := [][]int{}
	min_diff := -1

	sort.Slice(arr, func(i, j int) bool {
		return arr[i] < arr[j]
	})

	for i := 0; i < len(arr)-1; i++ {
		a, b := arr[i], arr[i+1]
		diff := b - a

		if min_diff < 0 {
			min_diff = diff
		}

		if min_diff > diff {
			min_diff = diff
			res = [][]int{[]int{a, b}}
		} else if min_diff == diff {
			res = append(res, []int{a, b})
		}
	}

	return res
}

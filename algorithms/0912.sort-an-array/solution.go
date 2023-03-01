func sortArray(nums []int) []int {
	return mergeSort(nums)
}

func mergeSort(arr []int) (res []int) {
	if len(arr) <= 1 {
		return arr
	}

	pivot := len(arr) / 2
	left := mergeSort(arr[:pivot])
	right := mergeSort(arr[pivot:])

	l, r := 0, 0

	for l < len(left) && r < len(right) {
		if left[l] < right[r] {
			res = append(res, left[l])
			l++
		} else {
			res = append(res, right[r])
			r++
		}
	}

	for l < len(left) {
		res = append(res, left[l])
		l++
	}

	for r < len(right) {
		res = append(res, right[r])
		r++
	}

	return
}

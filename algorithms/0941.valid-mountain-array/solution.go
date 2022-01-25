func validMountainArray(arr []int) bool {
	if len(arr) < 3 {
		return false
	}

	i := 1
	for i < len(arr) {
		if arr[i] <= arr[i-1] {
			break
		}
		i++
	}

	if i == len(arr) || i == 1 {
		return false
	}

	for i < len(arr) {
		if arr[i] >= arr[i-1] {
			return false
		}
		i++
	}

	return true
}

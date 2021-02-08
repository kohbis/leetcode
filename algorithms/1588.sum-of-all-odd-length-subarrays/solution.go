func sumOddLengthSubarrays(arr []int) int {
	count := 0

	for i := 1; i <= len(arr); i += 2 {
		for j := 0; j < len(arr)-i+1; j++ {
			for k := j; k < i+j; k++ {
				count += arr[k]
			}
		}
	}
	return count
}

import "sort"

func trimMean(arr []int) float64 {
	l := len(arr)
	n := int(float64(l) * 0.05)

	sort.Ints(arr)
	sum := 0
	for i := n; i < (l - n); i++ {
		sum += arr[i]
	}

	return float64(sum) / float64(l-n*2)
}

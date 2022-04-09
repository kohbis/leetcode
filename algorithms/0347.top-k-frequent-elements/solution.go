func topKFrequent(nums []int, k int) []int {
	m := map[int]int{}
	for _, num := range nums {
		m[num]++
	}

	type count struct {
		Key   int
		Value int
	}

	var numsCount []count
	for k, v := range m {
		numsCount = append(numsCount, count{k, v})
	}
	sort.Slice(numsCount, func(i, j int) bool {
		return numsCount[i].Value > numsCount[j].Value
	})

	res := []int{}
	for _, x := range numsCount {
		if len(res) < k {
			res = append(res, x.Key)
		} else {
			break
		}
	}

	return res
}

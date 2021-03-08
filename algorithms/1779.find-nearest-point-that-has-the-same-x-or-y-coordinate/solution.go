func nearestValidPoint(x int, y int, points [][]int) int {
	dists := [][]int{}

	for idx, point := range points {
		px := point[0]
		py := point[1]

		if x == px || y == py {
			dx := math.Abs(float64(x - px))
			dy := math.Abs(float64(y - py))
			dists = append(dists, []int{int(dx + dy), idx})
		}
	}

	if len(dists) == 0 {
		return -1
	}

	sort.Slice(dists, func(i, j int) bool {
		if dists[i][0] == dists[j][0] {
			return dists[i][1] < dists[j][1]
		}

		return dists[i][0] < dists[j][0]
	})

	fmt.Println(dists)

	return dists[0][1]
}

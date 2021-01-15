func maximumUnits(boxTypes [][]int, truckSize int) int {
	res := 0

	sort.Slice(boxTypes, func(i, j int) bool {
		return boxTypes[i][1] > boxTypes[j][1]
	})

	for _, boxType := range boxTypes {
		num, unit := boxType[0], boxType[1]

		if num > truckSize {
			res += truckSize * unit
			break
		} else {
			res += num * unit
			truckSize -= num
		}
	}

	return res
}

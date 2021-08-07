func slowestKey(releaseTimes []int, keysPressed string) byte {
	slowestTime := releaseTimes[0]
	slowestKey := keysPressed[0]

	for i := 1; i < len(releaseTimes); i++ {
		time := releaseTimes[i] - releaseTimes[i-1]

		if time > slowestTime {
			slowestTime = time
			slowestKey = keysPressed[i]
		} else if time == slowestTime && keysPressed[i] > slowestKey {
			slowestKey = keysPressed[i]
		}
	}

	return slowestKey
}

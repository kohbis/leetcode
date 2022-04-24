type StationChecks struct {
	startStation string
	checkInTime  int
}

type UndergroundSystem struct {
	cardIds map[int]StationChecks
	times   map[[2]string][]int
}

func Constructor() UndergroundSystem {
	return UndergroundSystem{map[int]StationChecks{}, map[[2]string][]int{}}
}

func (this *UndergroundSystem) CheckIn(id int, stationName string, t int) {
	this.cardIds[id] = StationChecks{stationName, t}
}

func (this *UndergroundSystem) CheckOut(id int, stationName string, t int) {
	check := this.cardIds[id]
	key := [2]string{check.startStation, stationName}
	this.times[key] = append(this.times[key], t-check.checkInTime)
}

func (this *UndergroundSystem) GetAverageTime(startStation string, endStation string) float64 {
	key := [2]string{startStation, endStation}
	sum := 0
	for _, time := range this.times[key] {
		sum += time
	}

	return float64(sum) / float64(len(this.times[key]))
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * obj := Constructor();
 * obj.CheckIn(id,stationName,t);
 * obj.CheckOut(id,stationName,t);
 * param_3 := obj.GetAverageTime(startStation,endStation);
 */

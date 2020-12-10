type ParkingSystem struct {
	parkingSpaces []int
}


func Constructor(big int, medium int, small int) ParkingSystem {
	return ParkingSystem { []int{ 0, big, medium, small } }
}


func (this *ParkingSystem) AddCar(carType int) bool {
	if this.parkingSpaces[carType] == 0 {
		return false
	}
	this.parkingSpaces[carType] -= 1
	return true
}


/**
 * Your ParkingSystem object will be instantiated and called as such:
 * obj := Constructor(big, medium, small);
 * param_1 := obj.AddCar(carType);
 */

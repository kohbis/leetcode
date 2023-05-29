class ParkingSystem {
  parkingSpaces = [0, 0, 0];

  constructor(big: number, medium: number, small: number) {
    this.parkingSpaces = [big, medium, small];
  }

  addCar(carType: number): boolean {
    if (this.parkingSpaces[carType - 1] > 0) {
      this.parkingSpaces[carType - 1]--;
      return true;
    } else {
      return false;
    }
  }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * var obj = new ParkingSystem(big, medium, small)
 * var param_1 = obj.addCar(carType)
 */

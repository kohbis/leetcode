type MyHashSet struct {
	container []bool
}

func Constructor() MyHashSet {
	return MyHashSet{container: make([]bool, 1000001)}
}

func (this *MyHashSet) Add(key int) {
	this.container[key] = true
}

func (this *MyHashSet) Remove(key int) {
	this.container[key] = false
}

func (this *MyHashSet) Contains(key int) bool {
	return this.container[key]
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Add(key);
 * obj.Remove(key);
 * param_3 := obj.Contains(key);
 */

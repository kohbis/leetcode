type OrderedStream struct {
	stream []string
	ptr    int
}

func Constructor(n int) OrderedStream {
	return OrderedStream{make([]string, n), 0}
}

func (this *OrderedStream) Insert(id int, value string) []string {
	chunkList := []string{}
	this.stream[id-1] = value

	for this.ptr < len(this.stream) && this.stream[this.ptr] != "" {
		chunkList = append(chunkList, this.stream[this.ptr])
		this.ptr += 1
	}

	return chunkList
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * obj := Constructor(n);
 * param_1 := obj.Insert(id,value);
 */

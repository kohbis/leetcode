from threading import Thread, Lock

class Foo(object):
    def __init__(self):
        self.lock_second = Lock()
        self.lock_third = Lock()

        self.lock_second.acquire()
        self.lock_third.acquire()


    def first(self, printFirst):
        """
        :type printFirst: method
        :rtype: void
        """
        
        # printFirst() outputs "first". Do not change or remove this line.
        printFirst()
        self.lock_second.release()


    def second(self, printSecond):
        """
        :type printSecond: method
        :rtype: void
        """
        self.lock_second.acquire()
        # printSecond() outputs "second". Do not change or remove this line.
        printSecond()
        self.lock_third.release()


    def third(self, printThird):
        """
        :type printThird: method
        :rtype: void
        """
        self.lock_third.acquire()
        # printThird() outputs "third". Do not change or remove this line.
        printThird()

# [155. Min Stack](https://leetcode.com/problems/min-stack)

## Mô tả

Thiết kế một ngăn xếp hỗ trợ các thao tác push, pop, top và truy xuất phần tử nhỏ nhất trong thời gian `O(1)`.

Triển khai lớp `MinStack`:
<ul>
	<li><code>MinStack()</code> khởi tạo đối tượng ngăn xếp.</li>
	<li><code>void push(val)</code> đưa phần tử <code>val</code> vào đầu ngăn xếp.</li>
	<li><code>void pop()</code> xoá phần tử đầu khỏi ngăn xếp.</li>
	<li><code>int top()</code> lấy giá trị phần tử đầu từ ngăn xếp.</li>
	<li><code>int getMin()</code> truy xuất giá trị nhỏ nhất trong ngắn xếp.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Ví dụ 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;MinStack&quot;,&quot;push&quot;,&quot;push&quot;,&quot;push&quot;,&quot;getMin&quot;,&quot;pop&quot;,&quot;top&quot;,&quot;getMin&quot;]
[[],[-2],[0],[-3],[],[],[],[]]

<strong>Output</strong>
[null,null,null,null,-3,null,0,-2]

<strong>Explanation</strong>
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin(); // return -3
minStack.pop();
minStack.top();    // return 0
minStack.getMin(); // return -2
</pre>

<p>&nbsp;</p>
<p><strong>Yêu cầu:</strong></p>

<ul>
	<li><code>-2<sup>31</sup> &lt;= val &lt;= 2<sup>31</sup> - 1</code></li>
	<li>Phương thức <code>pop</code>, <code>top</code> và <code>getMin</code> luôn được gọi trên một ngăn xếp <strong>có-phần tử</strong>.</li>
	<li>Ít nhất có <code>3 * 10<sup>4</sup></code> lần gọi đến <code>push</code>, <code>pop</code>, <code>top</code>, và <code>getMin</code>.</li>
</ul>

## Giải pháp

<!-- tabs:start -->

### **Python3**

```python
class MinStack:

    def __init__(self):
        """
        initialize your data structure here.
        """
        self.s = []
        self.mins = [float('inf')]

    def push(self, val: int) -> None:
        self.s.append(val)
        self.mins.append(min(self.mins[-1], val))

    def pop(self) -> None:
        self.s.pop()
        self.mins.pop()

    def top(self) -> int:
        return self.s[-1]

    def getMin(self) -> int:
        return self.mins[-1]


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(val)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()
```

### **Java**

```java
class MinStack {
    private Deque<Integer> s;
    private Deque<Integer> mins;

    /** initialize your data structure here. */
    public MinStack() {
        s = new ArrayDeque<>();
        mins = new ArrayDeque<>();
        mins.push(Integer.MAX_VALUE);
    }

    public void push(int val) {
        s.push(val);
        mins.push(Math.min(mins.peek(), val));
    }

    public void pop() {
        s.pop();
        mins.pop();
    }

    public int top() {
        return s.peek();
    }

    public int getMin() {
        return mins.peek();
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack obj = new MinStack();
 * obj.push(val);
 * obj.pop();
 * int param_3 = obj.top();
 * int param_4 = obj.getMin();
 */
```

### **C++**

```cpp
class MinStack {
private:
    stack<int> s;
    stack<int> mins;

public:
    /** initialize your data structure here. */
    MinStack() {
        mins.push(INT_MAX);
    }

    void push(int val) {
        s.push(val);
        mins.push(min(mins.top(), val));
    }

    void pop() {
        s.pop();
        mins.pop();
    }

    int top() {
        return s.top();
    }

    int getMin() {
        return mins.top();
    }
};

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack* obj = new MinStack();
 * obj->push(val);
 * obj->pop();
 * int param_3 = obj->top();
 * int param_4 = obj->getMin();
 */
```

### **JavaScript**

```js
/**
 * initialize your data structure here.
 */
var MinStack = function () {
    this.s = [];
    this.mins = [Infinity];
};

/**
 * @param {number} val
 * @return {void}
 */
MinStack.prototype.push = function (val) {
    this.s.push(val);
    this.mins.push(Math.min(this.mins[this.mins.length - 1], val));
};

/**
 * @return {void}
 */
MinStack.prototype.pop = function () {
    this.s.pop();
    this.mins.pop();
};

/**
 * @return {number}
 */
MinStack.prototype.top = function () {
    return this.s[this.s.length - 1];
};

/**
 * @return {number}
 */
MinStack.prototype.getMin = function () {
    return this.mins[this.mins.length - 1];
};

/**
 * Your MinStack object will be instantiated and called as such:
 * var obj = new MinStack()
 * obj.push(val)
 * obj.pop()
 * var param_3 = obj.top()
 * var param_4 = obj.getMin()
 */
```

### **...**

```

```

<!-- tabs:end -->

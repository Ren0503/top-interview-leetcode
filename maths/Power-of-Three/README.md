# [326. Power of Three](https://leetcode.com/problems/power-of-three)

## Mô tả

Cho một số nguyên `n`, trả về `true` nếu nó là lũy thừa của 3. Ngược lại trả về `false`. Một số nguyên `n` là luỹ thừa của 3, nếu tồn tại số nguyên `x` sao cho <code>n == 3<sup>x</sup></code>.</p>

<p>&nbsp;</p>
<p><strong>Ví dụ 1:</strong></p>
<pre><strong>Input:</strong> n = 27
<strong>Output:</strong> true
</pre><p><strong>Ví dụ 2:</strong></p>
<pre><strong>Input:</strong> n = 0
<strong>Output:</strong> false
</pre><p><strong>Ví dụ 3:</strong></p>
<pre><strong>Input:</strong> n = 9
<strong>Output:</strong> true
</pre><p><strong>Ví dụ 4:</strong></p>
<pre><strong>Input:</strong> n = 45
<strong>Output:</strong> false
</pre>
<p>&nbsp;</p>
<p><strong>Yêu cầu:</strong></p>

<ul>
	<li><code>-2<sup>31</sup> &lt;= n &lt;= 2<sup>31</sup> - 1</code></li>
</ul>

<p>&nbsp;</p>
<strong>Thử thách:</strong> Có thể thực hiện mà không cần vòng lặp/đệ quy không?

## Giải pháp

<!-- tabs:start -->

### **Python3**

```python

```

### **Java**

```java

```

### **TypeScript**

```ts
function isPowerOfThree(n: number): boolean {
    while (n > 2) {
        if (n % 3) return false;
        n /= 3;
    }
    return n == 1;
}
```

### **...**

```

```

<!-- tabs:end -->

# [204. Count Primes](https://leetcode.com/problems/count-primes)

## Mô tả

Đếm số lượng số nguyên tố nhỏ hơn một số nguyên `n` không âm.

<p>&nbsp;</p>
<p><strong>Ví dụ 1:</strong></p>

<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 4
</pre>
<strong>Giải thích:</strong> Có 4 số nguyên tố nhỏ hơn 10 là 2, 3, 5, 7.

<p><strong>Ví dụ 2:</strong></p>

<pre>
<strong>Input:</strong> n = 0
<strong>Output:</strong> 0
</pre>

<p><strong>Ví dụ 3:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Yêu cầu:</strong></p>

<ul>
	<li><code>0 &lt;= n &lt;= 5 * 10<sup>6</sup></code></li>
</ul>

## Giải pháp

<!-- tabs:start -->

### **Python3**

```python
class Solution:
    def countPrimes(self, n: int) -> int:
        if n < 2:
            return 0
        res = 0
        primes = [True for _ in range(n)]
        for i in range(2, n):
            if primes[i]:
                res += 1
                for j in range(i * i, n, i):
                    primes[j] = False
        return res
```

### **Java**

```java
class Solution {
    public int countPrimes(int n) {
        if (n < 2) return 0;
        boolean[] primes = new boolean[n];
        Arrays.fill(primes, true);
        int res = 0;
        for (int i = 2; i < n; ++i) {
            if (primes[i]) {
                ++res;
                if ((long) i * i < n) {
                    for (int j = i * i; j < n; j += i) {
                        primes[j] = false;
                    }
                }
            }
        }
        return res;
    }
}
```

### **...**

```

```

<!-- tabs:end -->

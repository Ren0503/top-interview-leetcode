# [204. Count Primes](https://leetcode.com/problems/count-primes)

## Description

<p>Count the number of prime numbers less than a non-negative number, <code>n</code>.</p>

<p>&nbsp;</p>
<p><strong>Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
</pre>

<p><strong>Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 0
<strong>Output:</strong> 0
</pre>

<p><strong>Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= n &lt;= 5 * 10<sup>6</sup></code></li>
</ul>

## Solutions

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

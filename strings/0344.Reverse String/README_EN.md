# [344. Reverse String](https://leetcode.com/problems/reverse-string)

[中文文档](/solution/0300-0399/0344.Reverse%20String/README.md)

## Description

<p>Write a function that reverses a string. The input string is given as an array of characters <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong>Example 1:</strong></p>
<pre><strong>Input:</strong> s = ["h","e","l","l","o"]
<strong>Output:</strong> ["o","l","l","e","h"]
</pre><p><strong>Example 2:</strong></p>
<pre><strong>Input:</strong> s = ["H","a","n","n","a","h"]
<strong>Output:</strong> ["h","a","n","n","a","H"]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s[i]</code> is a <a href="https://en.wikipedia.org/wiki/ASCII#Printable_characters" target="_blank">printable ascii character</a>.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong> Do not allocate extra space for another array. You must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with <code>O(1)</code> extra memory.</p>

## Solutions

<!-- tabs:start -->

### **Python3**

```python
class Solution:
    def reverseString(self, s: List[str]) -> None:
        """
        Do not return anything, modify s in-place instead.
        """
        s[:] = s[::-1]
```

### **Java**

```java
class Solution {
    public void reverseString(char[] s) {
        for (int i = 0, j = s.length - 1; i < j; ++i, --j) {
            char t = s[i];
            s[i] = s[j];
            s[j] = t;
        }
    }
}
```

### **C++**

```cpp
class Solution {
public:
    void reverseString(vector<char>& s) {
        for (int i = 0, j = s.size() - 1; i < j; ++i, --j)
            swap(s[i], s[j]);
    }
};
```

### **Go**

```go
func reverseString(s []byte) {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}
```

### **JavaScript**

```js
/**
 * @param {character[]} s
 * @return {void} Do not return anything, modify s in-place instead.
 */
var reverseString = function (s) {
    for (let i = 0, j = s.length - 1; i < j; ++i, --j) {
        [s[i], s[j]] = [s[j], s[i]];
    }
};
```

### **...**

```

```

<!-- tabs:end -->

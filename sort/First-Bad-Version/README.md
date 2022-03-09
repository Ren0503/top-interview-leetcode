# [278. First Bad Version](https://leetcode.com/problems/first-bad-version)

## Mô tả

Bạn là một nhà quản lý sản phẩm và hiện đang lãnh đạo một team phát triển sản phẩm mới. Không may, phiên bản cuối cùng trong sản phẩm của bạn đã bị lỗi khi kiểm tra chất lượng. Vì mỗi phiên bản đều được phát triển dựa trên phiên bản trước đó. Nên tất cả phiên bản sau một phiên bản lỗi đều bị lỗi tương tự.

Giả sử bạn có `n` phiên bản `[1, 2, ..., n]`  và bạn muốn tìm cái bị lỗi đầu tiên, cái dẫn đến tất cả sản phẩm phía sau bị lỗi.

Bạn được cho API `bool isBadVersion(version)` trả về bất cứ `version` nào là lỗi. Triển khai một hàm để phát hiện phiên bản lỗi đầu tiên. Bạn nên hạn chế tối thiểu số lần gọi API.

<p>&nbsp;</p>
<p><strong>Ví dụ 1:</strong></p>

<pre>
<strong>Input:</strong> n = 5, bad = 4
<strong>Output:</strong> 4
</pre>

<strong>Giải thích:</strong>
gọi isBadVersion(3) -&gt; false;
isBadVersion(5)&nbsp;-&gt; true;
isBadVersion(4)&nbsp;-&gt; true
Do đó 4 là phiên bản lỗi đầu tiên.

<p><strong>Ví dụ 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1, bad = 1
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Yêu cầu:</strong></p>

<ul>
	<li><code>1 &lt;= bad &lt;= n &lt;= 2<sup>31</sup> - 1</code></li>
</ul>

## Giải pháp

<!-- tabs:start -->

### **Python3**

```python
# The isBadVersion API is already defined for you.
# @param version, an integer
# @return an integer
# def isBadVersion(version):

class Solution:
    def firstBadVersion(self, n):
        """
        :type n: int
        :rtype: int
        """
        left, right = 1, n
        while left < right:
            mid  = (left + right) >> 1
            if isBadVersion(mid):
                right = mid
            else:
                left = mid + 1
        return left
```

### **Java**

```java
/* The isBadVersion API is defined in the parent class VersionControl.
      boolean isBadVersion(int version); */

public class Solution extends VersionControl {
    public int firstBadVersion(int n) {
        int left = 1, right = n;
        while (left < right) {
            int mid = (left + right) >>> 1;
            if (isBadVersion(mid)) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
}
```

### **C++**

```cpp
// The API isBadVersion is defined for you.
// bool isBadVersion(int version);

class Solution {
public:
    int firstBadVersion(int n) {
        int left = 1, right = n;
        while (left < right) {
            int mid = left + ((right - left) >> 1);
            if (isBadVersion(mid)) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
};
```

### **JavaScript**

```js
/**
 * Definition for isBadVersion()
 *
 * @param {integer} version number
 * @return {boolean} whether the version is bad
 * isBadVersion = function(version) {
 *     ...
 * };
 */

/**
 * @param {function} isBadVersion()
 * @return {function}
 */
var solution = function (isBadVersion) {
    /**
     * @param {integer} n Total versions
     * @return {integer} The first bad version
     */
    return function (n) {
        let left = 1;
        let right = n;
        while (left < right) {
            const mid = (left + right) >>> 1;
            if (isBadVersion(mid)) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    };
};
```

### **Go**

```go
/**
 * Forward declaration of isBadVersion API.
 * @param   version   your guess about first bad version
 * @return 	 	      true if current version is bad
 *			          false if current version is good
 * func isBadVersion(version int) bool;
 */

func firstBadVersion(n int) int {
	left, right := 1, n
	for left < right {
		mid := (left + right) >> 1
		if isBadVersion(mid) {
			right = mid
		} else {
			left = mid + 1
		}
	}
	return left
}
```

### **...**

```

```

<!-- tabs:end -->

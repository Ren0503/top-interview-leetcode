# [88. Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array)

## Mô tả

Cho hai mảng số nguyên đã sắp xếp là `nums1` và `nums2`, ghép `nums2` vào `nums1` như một mảng đã sắp xếp.

Số lượng phần tử ban đầu của `nums1` và `nums2` là `m` và `n`. Giả sử rằng kích thước của `nums1` bằng với `m + 1` đủ để chứa các phần tử thêm vào từ `nums2`.

<p>&nbsp;</p>
<p><strong>Ví dụ 1:</strong></p>
<pre><strong>Input:</strong> nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
<strong>Output:</strong> [1,2,2,3,5,6]
</pre><p><strong>Ví dụ 2:</strong></p>
<pre><strong>Input:</strong> nums1 = [1], m = 1, nums2 = [], n = 0
<strong>Output:</strong> [1]
</pre>
<p><strong>Yêu cầu:</strong></p>

<ul>
	<li><code>nums1.length == m + n</code></li>
	<li><code>nums2.length == n</code></li>
	<li><code>0 &lt;= m, n &lt;= 200</code></li>
	<li><code>1 &lt;= m + n &lt;= 200</code></li>
	<li><code>-10<sup>9</sup> &lt;= nums1[i], nums2[i] &lt;= 10<sup>9</sup></code></li>
</ul>

## Giải pháp

<!-- tabs:start -->

### **Python3**

```python
class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        i, j, k = m - 1, n - 1, m + n - 1
        while j >= 0:
            if i >= 0 and nums1[i] > nums2[j]:
                nums1[k] = nums1[i]
                i -= 1
            else:
                nums1[k] = nums2[j]
                j -= 1
            k -= 1
```

### **Java**

```java
class Solution {
    public void merge(int[] nums1, int m, int[] nums2, int n) {
        int i = m - 1, j = n - 1, k = m + n - 1;
        while (j >= 0) {
            if (i >= 0 && nums1[i] > nums2[j]) {
                nums1[k--] = nums1[i--];
            } else {
                nums1[k--] = nums2[j--];
            }
        }
    }
}
```

### **Go**

```go
func merge(nums1 []int, m int, nums2 []int, n int)  {
    i, j, k := m - 1, n - 1, m + n - 1
    for j >= 0 {
        if i >= 0 && nums1[i] > nums2[j] {
            nums1[k] = nums1[i]
            i--
        } else {
            nums1[k] = nums2[j]
            j--
        }
        k--
    }
}
```

### **JavaScript**

```js
/**
 * @param {number[]} nums1
 * @param {number} m
 * @param {number[]} nums2
 * @param {number} n
 * @return {void} Do not return anything, modify nums1 in-place instead.
 */
var merge = function (nums1, m, nums2, n) {
    let i = m - 1,
        j = n - 1,
        k = m + n - 1;
    while (j >= 0) {
        if (i >= 0 && nums1[i] > nums2[j]) {
            nums1[k--] = nums1[i--];
        } else {
            nums1[k--] = nums2[j--];
        }
    }
};
```

### **...**

```

```

<!-- tabs:end -->

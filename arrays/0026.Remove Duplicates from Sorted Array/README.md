# [26. Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array)

## Mô tả

Cho một mảng đã sắp xếp `nums`, xoá các phần tử trùng lặp sao cho tất cả phần tử chỉ xuất hiện **một lần** và trả về độ dài mới của mảng.

Không được cấp phát bộ nhớ bổ sung cho các mảng khác, bạn phải thực hiện thay đổi lên chính mảng đầu vào, với độ phức tạp không gian là O(1).

### Làm rõ

Tại sao giá trị trả về là một số nguyên nhưng câu trả lời cho vấn đề lại là một mảng?

Lưu ý rằng mảng đầu vào được truyền qua bằng tham chiếu, có nghĩa là mọi sửa đổi cho mảng đầu vào sẽ được thực hiện lên chính mảng đó.

### Ví dụ 1

```
Input: nums = [1, 1, 2]
Output: 2, nums = [1, 2]
```

Hàm của bạn trả về length = 2, với hai phần tử trong mảng `nums` là `1` và `2`.

### Ví dụ 2

```
Input: nums = [0,0,1,1,1,2,2,3,3,4]
Output: 5, nums = [0,1,2,3,4]
```

Hàm trả về length = 5, với 5 phần tử trong mảng `nums` đã được chỉnh sửa là `1`,`2`, `3`, `4` và `5`. 

## Yêu cầu

<ul>
	<li><code>0 &lt;= nums.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>nums</code>&nbsp;được sắp xếp theo thứ tự lớn dần đều.</li>
</ul>

## Giải pháp

<!-- tabs:start -->

### **Python3**

```python
class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        i = 0
        for num in nums:
            if i < 1 or num != nums[i - 1]:
                nums[i] = num
                i += 1
        return i
```

### **Java**

```java
class Solution {
    public int removeDuplicates(int[] nums) {
        int i = 0;
        for (int num : nums) {
            if (i < 1 || num != nums[i - 1]) {
                nums[i++] = num;
            }
        }
        return i;
    }
}
```

### **C++**

```cpp
class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int i = 0;
        for (int& num : nums)
            if (i < 1 || num != nums[i - 1])
                nums[i++] = num;
        return i;
    }
};
```

```cpp
class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        nums.erase(unique(nums.begin(), nums.end()), nums.end());
        return nums.size();
    }
};
```

### **Go**

```go
func removeDuplicates(nums []int) int {
    i := 0
	for _, num := range nums {
		if i < 1 || num != nums[i-1] {
			nums[i] = num
			i++
		}
	}
	return i
}
```

### **JavaScript**

```js
/**
 * @param {number[]} nums
 * @return {number}
 */
var removeDuplicates = function (nums) {
    let i = 0;
    for (const num of nums) {
        if (i < 1 || num != nums[i - 1]) {
            nums[i++] = num;
        }
    }
    return i;
};
```

### **C#**

```cs
public class Solution {
    public int RemoveDuplicates(int[] nums) {
        int i = 0;
        foreach(int num in nums)
        {
            if (i < 1 || num != nums[i - 1])
            {
                nums[i++] = num;
            }
        }
        return i;
    }
}
```

### **Rust**

```rust
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = 0;
        for i in 0..nums.len() {
            if i == 0 || nums[i] != nums[len - 1] {
                nums[len] = nums[i];
                len += 1;
            }
        }
        len as i32
    }
}
```

### **...**

```

```

<!-- tabs:end -->

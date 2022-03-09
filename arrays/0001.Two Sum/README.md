# [1. Two Sum](https://leetcode.com/problems/two-sum)

## Mô tả

Cho một mảng số nguyên `nums` và một giá trị nguyên `target`, trả về hai phần tử trong mảng mà tổng của chúng bằng với `target`.

Giả sử mỗi input đều có một giải pháp chính xác, và bạn không thể dùng một phần tử hai lần.

### Ví dụ 1

```
Input: nums = [2, 7, 11, 15], target = 9
Output: [0, 1]
```

Vì nums[0] + nums[1] == 9, ta trả về [0, 1].

### Ví dụ 2

```
Input: nums = [3, 2, 4], target = 6
Output: [1, 2]
```

### Ví dụ 3

```
Input: nums = [3, 3], target = 6
Output: [0, 1]
```

## Yêu cầu

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>3</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= target &lt;= 10<sup>9</sup></code></li>
	<li><strong>Chỉ có một câu trả lời hợp lệ.</strong></li>
</ul>

## Giải pháp


<!-- tabs:start -->

### **Python3**

```python
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        helper = {}
        for i, v in enumerate(nums):
            num = target - v
            if num in helper:
                return [helper[num], i]
            helper[v] = i
```

### **Java**

```java
class Solution {
    public int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> map = new HashMap<>();
        for (int i = 0; i < nums.length; ++i) {
            int num = target - nums[i];
            if (map.containsKey(num)) {
                return new int[]{map.get(num), i};
            }
            map.put(nums[i], i);
        }
        return null;
    }
}
```

### **C++**

```cpp
class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> map;
        for (int i = 0; i < nums.size(); ++i) {
            int num = target - nums[i];
            if (map.find(num) != map.end()) {
                return {map[num], i};
            }
            map[nums[i]] = i;
        }
        return {};
    }
};
```

### **Go**

```go
func twoSum(nums []int, target int) []int {
	numMap := make(map[int]int)
	for i, num := range nums {
		other := target - num
		if _, ok := numMap[other]; ok {
			return []int{numMap[other], i}
		}
		numMap[num] = i
	}
	return nil
}
```

### **JavaScript**

```js
var twoSum = function (nums, target) {
    const map = new Map();
    for (let i = 0; i < nums.length; i++) {
        if (map.has(target - nums[i])) {
            return [map.get(target - nums[i]), i];
        }
        map.set(nums[i], i);
    }
    return [];
};
```

### **Swift**

```swift
class Solution {
    func twoSum(_ nums: [Int], _ target: Int) -> [Int] {
        var map = [Int: Int]()
        var i = 0
        for num in nums {
            map[num] = i
            i = i + 1
        }
        i = 0
        for num in nums {
            if let otherIndex = map[target - num], otherIndex != i {
                return [i, otherIndex]
            }
            i = i + 1
        }
        return []
    }
}
```

### **Nim**

```nim
import std/enumerate

proc twoSum(nums: seq[int], target: int): seq[int] =
    var
        bal: int
        tdx: int
    for idx, val in enumerate(nums):
        bal = target - val
        if bal in nums:
            tdx = nums.find(bal)
            if idx != tdx:
                return @[idx, tdx]

```

### **Rust**

```rust
use std::collections::HashMap;

pub fn soluation(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, item) in nums.iter().enumerate() {
        if map.contains_key(item) {
            return vec![i as i32, map[item]];
        } else {
            let x = target - nums[i];
            map.insert(x, i as i32);
        }
    }
    unreachable!()
}
```

### **...**

```

```

<!-- tabs:end -->

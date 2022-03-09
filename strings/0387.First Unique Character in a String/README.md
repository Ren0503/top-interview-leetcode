# [387. 字符串中的第一个唯一字符](https://leetcode-cn.com/problems/first-unique-character-in-a-string)

[English Version](/solution/0300-0399/0387.First%20Unique%20Character%20in%20a%20String/README_EN.md)

## 题目描述

<!-- 这里写题目描述 -->

<p>给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1。</p>

<p>&nbsp;</p>

<p><strong>示例：</strong></p>

<pre>s = &quot;leetcode&quot;
返回 0

s = &quot;loveleetcode&quot;
返回 2
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong>你可以假定该字符串只包含小写字母。</p>

## 解法

<!-- 这里可写通用的实现逻辑 -->

遍历字符串，用一个 map 或者字典存放字符串中每个字符出现的次数。然后再次遍历字符串，取出对应字符出现的次数，若次数为 1，直接返回当前字符串的下标。遍历结束，返回 -1。

<!-- tabs:start -->

### **Python3**

<!-- 这里可写当前语言的特殊实现逻辑 -->

```python
class Solution:
    def firstUniqChar(self, s: str) -> int:
        counter = Counter(s)
        for i, c in enumerate(s):
            if counter[c] == 1:
                return i
        return -1
```

### **Java**

<!-- 这里可写当前语言的特殊实现逻辑 -->

```java
class Solution {
    public int firstUniqChar(String s) {
        int[] counter = new int[26];
        for (char c : s.toCharArray()) {
            ++counter[c - 'a'];
        }
        for (int i = 0; i < s.length(); ++i) {
            char c = s.charAt(i);
            if (counter[c - 'a'] == 1) {
                return i;
            }
        }
        return -1;
    }
}
```

### **TypeScript**

```ts
function firstUniqChar(s: string): number {
    let record = new Map();
    for (let cur of [...s]) {
        record.set(cur, record.has(cur));
    }
    for (let i = 0; i < s.length; i++) {
        if (!record.get(s[i])) return i;
    }
    return -1;
}
```

### **C++**

```cpp
class Solution {
public:
    int firstUniqChar(string s) {
        vector<int> counter(26);
        for (char& c : s) ++counter[c - 'a'];
        for (int i = 0; i < s.size(); ++i)
            if (counter[s[i] - 'a'] == 1)
                return i;
        return -1;
    }
};
```

### **Go**

```go
func firstUniqChar(s string) int {
	counter := make([]int, 26)
	for _, c := range s {
		counter[c-'a']++
	}
	for i, c := range s {
		if counter[c-'a'] == 1 {
			return i
		}
	}
	return -1
}
```

### **JavaScript**

```js
/**
 * @param {string} s
 * @return {number}
 */
var firstUniqChar = function (s) {
    const counter = new Map();
    for (let c of s) {
        counter[c] = (counter[c] || 0) + 1;
    }
    for (let i = 0; i < s.length; ++i) {
        if (counter[s[i]] == 1) {
            return i;
        }
    }
    return -1;
};
```

### **...**

```

```

<!-- tabs:end -->

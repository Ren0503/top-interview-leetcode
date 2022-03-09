# [13. Roman to Integer](https://leetcode.com/problems/roman-to-integer)

## Mô tả

Số La Mã được biểu diễn bởi 7 ký tự khác nhau là: `I`, `V`, `X`, `L`, `C`, `D` và `M`.

<pre>

<strong>Symbol</strong>       <strong>Value</strong>

I             1

V             5

X             10

L             50

C             100

D             500

M             1000</pre>

Ví dụ, `2` được viết là `II` theo số La Mã, hai số 1 được đặt cùng với nhau. `12` được viết là `XII` đơn giản là `X + II`. Số `27` là `XXVII`, tức là `XX + V + II`.

Số la mã thường được theo chiều lớn nhất đến nhỏ nhất từ trái sang phải. Tuy nhiên, số bốn không phải là `IIII`. Thay vào đó nó được viết là `IV`. Bởi vì số năm trừ đi một là số bốn. Nguyên tắc tương tự được áp dụng cho số chín, `IX`. 6 đối tượng sử dụng nguyên tắc này là:
- `I` có thể đặt trước `v` (5) và `X` (10) để biểu diễn 4 và 9.
- `X` có thể đặt trước `L` (50) và `C` (100) để biểu diễn 40 và 90.
- `C` có thể đặt trước `D` (500) và `M` (1000) để biểu diễn 400 và 900.


Cho một số La Mã, hãy chuyển nó về số nguyên.

<p>&nbsp;</p>

<p><strong>Ví dụ 1:</strong></p>

<pre>

<strong>Input:</strong> s = &quot;III&quot;

<strong>Output:</strong> 3

</pre>

<p><strong>Ví dụ 2:</strong></p>

<pre>

<strong>Input:</strong> s = &quot;IV&quot;

<strong>Output:</strong> 4

</pre>

<p><strong>Ví dụ 3:</strong></p>

<pre>

<strong>Input:</strong> s = &quot;IX&quot;

<strong>Output:</strong> 9

</pre>

<p><strong>Ví dụ 4:</strong></p>

<pre>

<strong>Input:</strong> s = &quot;LVIII&quot;

<strong>Output:</strong> 58

<strong>Explanation:</strong> L = 50, V= 5, III = 3.

</pre>

<p><strong>Ví dụ 5:</strong></p>

<pre>

<strong>Input:</strong> s = &quot;MCMXCIV&quot;

<strong>Output:</strong> 1994

</pre>
<strong>Giải thích:</strong> M = 1000, CM = 900, XC = 90 và IV = 4.

<p>&nbsp;</p>

<p><strong>Yêu cầu:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 15</code></li>
	<li><code>s</code> chỉ bao gồm&nbsp;các ký tự <code>(&#39;I&#39;, &#39;V&#39;, &#39;X&#39;, &#39;L&#39;, &#39;C&#39;, &#39;D&#39;, &#39;M&#39;)</code>.</li>
	<li><strong>Đảm bảo</strong>&nbsp;rằng <code>s</code> là số La Mã hợp lệ trong khoảng <code>[1, 3999]</code>.</li>
</ul>

## Solutions

<!-- tabs:start -->

### **Python3**

```python
class Solution:
    def romanToInt(self, s: str) -> int:
        nums = {
            'M': 1000,
            'CM': 900,
            'D': 500,
            'CD': 400,
            'C': 100,
            'XC': 90,
            'L': 50,
            'XL': 40,
            'X': 10,
            'IX': 9,
            'V': 5,
            'IV': 4,
            'I': 1
        }
        i = res = 0
        while i < len(s):
            if i + 1 < len(s) and s[i:i + 2] in nums:
                res += nums[s[i: i + 2]]
                i += 2
            else:
                res += nums[s[i: i + 1]]
                i += 1
        return res
```

### **Java**

```java
class Solution {
    public int romanToInt(String s) {
        Map<String, Integer> nums = new HashMap<>();
        nums.put("M", 1000);
        nums.put("CM", 900);
        nums.put("D", 500);
        nums.put("CD", 400);
        nums.put("C", 100);
        nums.put("XC", 90);
        nums.put("L", 50);
        nums.put("XL", 40);
        nums.put("X", 10);
        nums.put("IX", 9);
        nums.put("V", 5);
        nums.put("IV", 4);
        nums.put("I", 1);
        int res = 0;
        for (int i = 0; i < s.length();) {
            if (i + 1 < s.length() && nums.get(s.substring(i, i + 2)) != null) {
                res += nums.get(s.substring(i, i + 2));
                i += 2;
            } else {
                res += nums.get(s.substring(i, i + 1));
                i += 1;
            }
        }
        return res;
    }
}
```

### **C++**

```cpp
class Solution {
   public:
    int romanToInt(string s) {
        unordered_map<char, int> nums{
            {'I', 1},
            {'V', 5},
            {'X', 10},
            {'L', 50},
            {'C', 100},
            {'D', 500},
            {'M', 1000},
        };
        int ans = 0;
        for (int i = 0; i < s.size() - 1; ++i) {
            if (nums[s[i]] < nums[s[i + 1]])
                ans -= nums[s[i]];
            else
                ans += nums[s[i]];
        }
        return ans + nums[s.back()];
    }
};
```

### **...**

```

```

<!-- tabs:end -->

# [412. Fizz Buzz](https://leetcode.com/problems/fizz-buzz)

## Mô tả

Cho một số nguyên `n`, trả về một mảng chuỗi `answer` trong đó:
- `answer[i] == "FizzBuzz"` nếu `i` chia hết cho 3 và 5.
- `answer[i] == "Fizz"` nếu `i` chia hết cho 3.
- `answer[i] == "Buzz"` nếu `i` chia hết cho 5.
- `answer[i] == i` nếu không có điều kiện nào thoả mãn.

<p>&nbsp;</p>
<p><strong>Ví dụ 1:</strong></p>
<pre><strong>Input:</strong> n = 3
<strong>Output:</strong> ["1","2","Fizz"]
</pre><p><strong>Ví dụ 2:</strong></p>
<pre><strong>Input:</strong> n = 5
<strong>Output:</strong> ["1","2","Fizz","4","Buzz"]
</pre><p><strong>Example 3:</strong></p>
<pre><strong>Input:</strong> n = 15
<strong>Output:</strong> ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
</pre>
<p>&nbsp;</p>
<p><strong>Yêu cầu:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>4</sup></code></li>
</ul>

## Giải pháp

<!-- tabs:start -->

### **Python3**

```python
class Solution:
    def fizzBuzz(self, n: int) -> List[str]:
        ans = []
        for i in range(1, n + 1):
            if i % 15 == 0:
                ans.append('FizzBuzz')
            elif i % 3 == 0:
                ans.append('Fizz')
            elif i % 5 == 0:
                ans.append('Buzz')
            else:
                ans.append(str(i))
        return ans
```

### **Java**

```java
class Solution {
    public List<String> fizzBuzz(int n) {
        List<String> ans = new ArrayList<>();
        for (int i = 1; i <= n; ++i) {
            String s = "";
            if (i % 3 == 0) {
                s += "Fizz";
            }
            if (i % 5 == 0) {
                s += "Buzz";
            }
            if (s.length() == 0) {
                s += i;
            }
            ans.add(s);
        }
        return ans;
    }
}
```

### **C++**

```cpp
class Solution {
public:
    vector<string> fizzBuzz(int n) {
        vector<string> ans;
        for (int i = 1; i <= n; ++i)
        {
            string s = "";
            if (i % 3 == 0) s += "Fizz";
            if (i % 5 == 0) s += "Buzz";
            if (s.size() == 0) s = to_string(i);
            ans.push_back(s);
        }
        return ans;
    }
};
```

### **Go**

```go
func fizzBuzz(n int) []string {
	var ans []string
	for i := 1; i <= n; i++ {
		s := &strings.Builder{}
		if i%3 == 0 {
			s.WriteString("Fizz")
		}
		if i%5 == 0 {
			s.WriteString("Buzz")
		}
		if s.Len() == 0 {
			s.WriteString(strconv.Itoa(i))
		}
		ans = append(ans, s.String())
	}
	return ans
}
```

### **...**

```

```

<!-- tabs:end -->

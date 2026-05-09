<a id="vect-vs-listt"></a>
## `Vec<T>`와 `List<T>`

> **학습할 내용:** `Vec<T>`와 `List<T>`, `HashMap`과 `Dictionary`, 안전한 접근 패턴
> (Rust가 예외를 던지지 않고 `Option`을 반환하는 이유), 그리고 컬렉션의 소유권 의미.
>
> **난이도:** 🟢 입문

`Vec<T>`는 Rust에서 C#의 `List<T>`에 해당하지만, 소유권 의미론이 함께 따라옵니다.

### C# `List<T>`
```csharp
// C# List<T> - 참조 타입, 힙 할당
var numbers = new List<int>();
numbers.Add(1);
numbers.Add(2);
numbers.Add(3);

// 메서드에 전달 - 참조가 복사됨
ProcessList(numbers);
Console.WriteLine(numbers.Count);  // 여전히 접근 가능

void ProcessList(List<int> list)
{
    list.Add(4);  // 원본 리스트 수정
    Console.WriteLine($"Count in method: {list.Count}");
}
```

### Rust `Vec<T>`
```rust
// Rust Vec<T> - 소유권을 가진 타입, 힙 할당
let mut numbers = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

// 소유권을 가져가는 함수
process_vec(numbers);
// println!("{:?}", numbers);  // ❌ 오류: numbers가 move됨

// 대여하는 함수
let mut numbers = vec![1, 2, 3];  // 편의를 위한 vec! 매크로
process_vec_borrowed(&mut numbers);
println!("{:?}", numbers);  // ✅ 여전히 접근 가능

fn process_vec(mut vec: Vec<i32>) {  // 소유권을 가져감
    vec.push(4);
    println!("Count in method: {}", vec.len());
    // vec는 여기서 drop됨
}

fn process_vec_borrowed(vec: &mut Vec<i32>) {  // 가변 대여
    vec.push(4);
    println!("Count in method: {}", vec.len());
}
```

### 벡터 생성과 초기화
```csharp
// C# List 초기화
var numbers = new List<int> { 1, 2, 3, 4, 5 };
var empty = new List<int>();
var sized = new List<int>(10);  // 초기 용량

// 다른 컬렉션에서 생성
var fromArray = new List<int>(new[] { 1, 2, 3 });
```

```rust
// Rust Vec 초기화
let numbers = vec![1, 2, 3, 4, 5];  // vec! 매크로
let empty: Vec<i32> = Vec::new();   // 비어 있을 때는 타입 표기가 필요
let sized = Vec::with_capacity(10); // 용량 미리 할당

// 이터레이터에서 생성
let from_range: Vec<i32> = (1..=5).collect();
let from_array = vec![1, 2, 3];
```

### 자주 쓰는 연산 비교
```csharp
// C# List 연산
var list = new List<int> { 1, 2, 3 };

list.Add(4);                    // 요소 추가
list.Insert(0, 0);              // 인덱스에 삽입
list.Remove(2);                 // 첫 번째 일치 항목 제거
list.RemoveAt(1);               // 인덱스 제거
list.Clear();                   // 모두 제거

int first = list[0];            // 인덱스 접근
int count = list.Count;         // 개수 가져오기
bool contains = list.Contains(3); // 포함 여부 확인
```

```rust
// Rust Vec 연산
let mut vec = vec![1, 2, 3];

vec.push(4);                    // 요소 추가
vec.insert(0, 0);               // 인덱스에 삽입
vec.retain(|&x| x != 2);        // 요소 제거(함수형 스타일)
vec.remove(1);                  // 인덱스 제거
vec.clear();                    // 모두 제거

let first = vec[0];             // 인덱스 접근(범위를 벗어나면 panic)
let safe_first = vec.get(0);    // 안전한 접근, Option<&T> 반환
let count = vec.len();          // 개수 가져오기
let contains = vec.contains(&3); // 포함 여부 확인
```

### 안전한 접근 패턴
```csharp
// C# - 예외 기반 범위 검사
public int SafeAccess(List<int> list, int index)
{
    try
    {
        return list[index];
    }
    catch (ArgumentOutOfRangeException)
    {
        return -1;  // 기본값
    }
}
```

```rust
// Rust - Option 기반 안전 접근
fn safe_access(vec: &Vec<i32>, index: usize) -> Option<i32> {
    vec.get(index).copied()  // Option<i32> 반환
}

fn main() {
    let vec = vec![1, 2, 3];
    
    // 안전한 접근 패턴
    match vec.get(10) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"),
    }
    
    // 또는 unwrap_or 사용
    let value = vec.get(10).copied().unwrap_or(-1);
    println!("Value: {}", value);
}
```

***

<a id="hashmap-vs-dictionary"></a>
## HashMap과 Dictionary

HashMap은 Rust에서 C#의 `Dictionary<K,V>`에 해당합니다.

### C# Dictionary
```csharp
// C# Dictionary<TKey, TValue>
var scores = new Dictionary<string, int>
{
    ["Alice"] = 100,
    ["Bob"] = 85,
    ["Charlie"] = 92
};

// 추가/갱신
scores["Dave"] = 78;
scores["Alice"] = 105;  // 기존 값 갱신

// 안전한 접근
if (scores.TryGetValue("Eve", out int score))
{
    Console.WriteLine($"Eve's score: {score}");
}
else
{
    Console.WriteLine("Eve not found");
}

// 순회
foreach (var kvp in scores)
{
    Console.WriteLine($"{kvp.Key}: {kvp.Value}");
}
```

### Rust의 HashMap
```rust
use std::collections::HashMap;

// HashMap 생성과 초기화
let mut scores = HashMap::new();
scores.insert("Alice".to_string(), 100);
scores.insert("Bob".to_string(), 85);
scores.insert("Charlie".to_string(), 92);

// 또는 이터레이터에서 생성
let scores: HashMap<String, i32> = [
    ("Alice".to_string(), 100),
    ("Bob".to_string(), 85),
    ("Charlie".to_string(), 92),
].into_iter().collect();

// 추가/갱신
let mut scores = scores;  // 가변으로 만들기
scores.insert("Dave".to_string(), 78);
scores.insert("Alice".to_string(), 105);  // 기존 값 갱신

// 안전한 접근
match scores.get("Eve") {
    Some(score) => println!("Eve's score: {}", score),
    None => println!("Eve not found"),
}

// 순회
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
```

### HashMap 연산
```csharp
// C# Dictionary 연산
var dict = new Dictionary<string, int>();

dict["key"] = 42;                    // 삽입/갱신
bool exists = dict.ContainsKey("key"); // 존재 여부 확인
bool removed = dict.Remove("key");    // 제거
dict.Clear();                        // 모두 비우기

// 기본값과 함께 가져오기
int value = dict.GetValueOrDefault("missing", 0);
```

```rust
use std::collections::HashMap;

// Rust HashMap 연산
let mut map = HashMap::new();

map.insert("key".to_string(), 42);   // 삽입/갱신
let exists = map.contains_key("key"); // 존재 여부 확인
let removed = map.remove("key");      // 제거, Option<V> 반환
map.clear();                         // 모두 비우기

// 고급 연산용 Entry API
let mut map = HashMap::new();
map.entry("key".to_string()).or_insert(42);  // 없으면 삽입
map.entry("key".to_string()).and_modify(|v| *v += 1); // 있으면 수정

// 기본값과 함께 가져오기
let value = map.get("missing").copied().unwrap_or(0);
```

### HashMap 키와 값의 소유권
```rust
// HashMap에서 소유권이 어떻게 동작하는지 이해하기
fn ownership_example() {
    let mut map = HashMap::new();
    
    // String 키와 값은 map 안으로 move됩니다.
    let key = String::from("name");
    let value = String::from("Alice");
    
    map.insert(key, value);
    // println!("{}", key);   // ❌ 오류: key가 move됨
    // println!("{}", value); // ❌ 오류: value가 move됨
    
    // 참조로 접근
    if let Some(name) = map.get("name") {
        println!("Name: {}", name);  // 값을 대여해서 사용
    }
}

// &str 키 사용(소유권 이전 없음)
fn string_slice_keys() {
    let mut map = HashMap::new();
    
    map.insert("name", "Alice");     // &str 키와 값
    map.insert("age", "30");
    
    // 문자열 리터럴은 소유권 문제가 없음
    println!("Name exists: {}", map.contains_key("name"));
}
```

***

## 컬렉션 다루기

### 순회 패턴
```csharp
// C# 순회 패턴
var numbers = new List<int> { 1, 2, 3, 4, 5 };

// 인덱스를 사용하는 for 루프
for (int i = 0; i < numbers.Count; i++)
{
    Console.WriteLine($"Index {i}: {numbers[i]}");
}

// foreach 루프
foreach (int num in numbers)
{
    Console.WriteLine(num);
}

// LINQ 메서드
var doubled = numbers.Select(x => x * 2).ToList();
var evens = numbers.Where(x => x % 2 == 0).ToList();
```

```rust
// Rust 순회 패턴
let numbers = vec![1, 2, 3, 4, 5];

// 인덱스를 함께 받는 for 루프
for (i, num) in numbers.iter().enumerate() {
    println!("Index {}: {}", i, num);
}

// 값들에 대한 for 루프
for num in &numbers {  // 각 요소를 대여
    println!("{}", num);
}

// 이터레이터 메서드(LINQ와 비슷함)
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();

// 또는 더 효율적으로, 소비하는 이터레이터 사용
let doubled: Vec<i32> = numbers.into_iter().map(|x| x * 2).collect();
```

### Iterator, IntoIterator, Iter 비교
```rust
// 서로 다른 순회 메서드 이해하기
fn iteration_methods() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // 1. iter() - 요소를 대여함(&T)
    for item in vec.iter() {
        println!("{}", item);  // item은 &i32
    }
    // 여기서 vec는 여전히 사용 가능
    
    // 2. into_iter() - 소유권을 가져감(T)
    for item in vec.into_iter() {
        println!("{}", item);  // item은 i32
    }
    // 여기서 vec는 더 이상 사용 불가
    
    let mut vec = vec![1, 2, 3, 4, 5];
    
    // 3. iter_mut() - 가변 대여(&mut T)
    for item in vec.iter_mut() {
        *item *= 2;  // item은 &mut i32
    }
    println!("{:?}", vec);  // [2, 4, 6, 8, 10]
}
```

### 결과 수집하기
```csharp
// C# - 오류가 있을 수 있는 컬렉션 처리
public List<int> ParseNumbers(List<string> inputs)
{
    var results = new List<int>();
    foreach (string input in inputs)
    {
        if (int.TryParse(input, out int result))
        {
            results.Add(result);
        }
        // 잘못된 입력은 조용히 건너뜀
    }
    return results;
}
```

```rust
// Rust - collect를 통한 명시적 오류 처리
fn parse_numbers(inputs: Vec<String>) -> Result<Vec<i32>, std::num::ParseIntError> {
    inputs.into_iter()
        .map(|s| s.parse::<i32>())  // Result<i32, ParseIntError> 반환
        .collect()                  // Result<Vec<i32>, ParseIntError>로 수집
}

// 대안: 오류를 걸러내기
fn parse_numbers_filter(inputs: Vec<String>) -> Vec<i32> {
    inputs.into_iter()
        .filter_map(|s| s.parse::<i32>().ok())  // Ok 값만 유지
        .collect()
}

fn main() {
    let inputs = vec!["1".to_string(), "2".to_string(), "invalid".to_string(), "4".to_string()];
    
    // 첫 번째 오류에서 실패하는 버전
    match parse_numbers(inputs.clone()) {
        Ok(numbers) => println!("All parsed: {:?}", numbers),
        Err(error) => println!("Parse error: {}", error),
    }
    
    // 오류를 건너뛰는 버전
    let numbers = parse_numbers_filter(inputs);
    println!("Successfully parsed: {:?}", numbers);  // [1, 2, 4]
}
```

---

## 연습문제

<details>
<summary><strong>🏋️ 연습문제: LINQ를 이터레이터로</strong> (펼쳐서 보기)</summary>

다음 C# LINQ 쿼리를 Rust다운 이터레이터 코드로 옮겨 보세요.

```csharp
var result = students
    .Where(s => s.Grade >= 90)
    .OrderByDescending(s => s.Grade)
    .Select(s => $"{s.Name}: {s.Grade}")
    .Take(3)
    .ToList();
```

다음 구조체를 사용하세요.
```rust
struct Student { name: String, grade: u32 }
```

성적이 90 이상인 학생 중 상위 3명을 `"Name: Grade"` 형식으로 담은 `Vec<String>`을 반환하세요.

<details>
<summary>🔑 해답</summary>

```rust
#[derive(Debug)]
struct Student { name: String, grade: u32 }

fn top_students(students: &mut [Student]) -> Vec<String> {
    students.sort_by(|a, b| b.grade.cmp(&a.grade)); // 내림차순 정렬
    students.iter()
        .filter(|s| s.grade >= 90)
        .take(3)
        .map(|s| format!("{}: {}", s.name, s.grade))
        .collect()
}

fn main() {
    let mut students = vec![
        Student { name: "Alice".into(), grade: 95 },
        Student { name: "Bob".into(), grade: 88 },
        Student { name: "Carol".into(), grade: 92 },
        Student { name: "Dave".into(), grade: 97 },
        Student { name: "Eve".into(), grade: 91 },
    ];
    let result = top_students(&mut students);
    assert_eq!(result, vec!["Dave: 97", "Alice: 95", "Carol: 92"]);
    println!("{result:?}");
}
```

**C#와의 핵심 차이:** Rust 이터레이터는 LINQ처럼 지연 평가되지만, `.sort_by()`는 즉시 실행되며 제자리에서 정렬합니다. 즉, 게으른 `OrderBy`는 없고, 먼저 정렬한 뒤 지연 연산을 이어 붙입니다.

</details>
</details>

***
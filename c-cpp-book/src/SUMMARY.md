# 요약

[서문](ch00-introduction.md)

---

# Part I — 기초

- [1. 소개와 동기](ch01-introduction-and-motivation.md)
    - [왜 C/C++ 개발자에게 Rust가 필요한가](ch01-1-why-c-cpp-developers-need-rust.md)
- [2. 시작하기](ch02-getting-started.md)
- [3. 내장 타입](ch03-built-in-types.md)
- [4. 제어 흐름](ch04-control-flow.md)
- [5. 자료구조](ch05-data-structures.md)
- [6. 열거형과 패턴 매칭](ch06-enums-and-pattern-matching.md)
- [7. 소유권과 대여](ch07-ownership-and-borrowing.md)
    - [라이프타임과 대여 심화](ch07-1-lifetimes-and-borrowing-deep-dive.md)
    - [스마트 포인터와 내부 가변성](ch07-2-smart-pointers-and-interior-mutability.md)
- [8. 크레이트와 모듈](ch08-crates-and-modules.md)
    - [테스트 패턴](ch08-1-testing-patterns.md)
- [9. 에러 처리](ch09-error-handling.md)
    - [에러 처리 모범 사례](ch09-1-error-handling-best-practices.md)
- [10. 트레잇](ch10-traits.md)
    - [제네릭](ch10-1-generics.md)
- [11. From과 Into 트레잇](ch11-from-and-into-traits.md)
- [12. 클로저](ch12-closures.md)
    - [이터레이터 활용 도구](ch12-1-iterator-power-tools.md)
- [13. 동시성](ch13-concurrency.md)
- [14. Unsafe Rust와 FFI](ch14-unsafe-rust-and-ffi.md)

---

# Part II — 심화

- [15. no_std — 표준 라이브러리 없는 Rust](ch15-no_std-rust-without-the-standard-library.md)
    - [임베디드 심화](ch15-1-embedded-deep-dive.md)
- [16. 사례 연구: 실제 C++에서 Rust로](ch16-case-studies.md)
    - [사례 연구 3-5: 라이프타임, 조합성, 트레잇 객체](ch16-cases-3-5-lifetime-borrowing.md)

---

# Part III — 모범 사례와 참고 자료

- [17. 모범 사례](ch17-best-practices.md)
    - [과도한 clone() 피하기](ch17-1-avoiding-excessive-clone.md)
    - [검사되지 않은 인덱싱 피하기](ch17-2-avoiding-unchecked-indexing.md)
    - [중첩된 대입 피라미드 줄이기](ch17-3-collapsing-assignment-pyramids.md)
    - [로깅과 트레이싱 생태계](ch17-4-logging-and-tracing-ecosystem.md)
- [18. C++ → Rust 의미론 심화](ch18-cpp-rust-semantic-deep-dives.md)
- [19. Rust 매크로: 전처리기에서 메타프로그래밍까지](ch19-macros.md)

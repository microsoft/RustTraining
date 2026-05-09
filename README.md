<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**라이선스** 이 프로젝트는 [MIT License](LICENSE)와 [Creative Commons Attribution 4.0 International (CC-BY-4.0)](LICENSE-DOCS)의 이중 라이선스로 배포됩니다.

</div>

<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**상표** 이 프로젝트에는 프로젝트, 제품 또는 서비스의 상표나 로고가 포함될 수 있습니다. Microsoft 상표 또는 로고의 허용된 사용은 [Microsoft Trademark & Brand Guidelines](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general)를 따라야 합니다. 이 프로젝트를 수정한 버전에서 Microsoft 상표나 로고를 사용할 경우, Microsoft의 후원 또는 공식 관계가 있는 것처럼 혼동을 일으켜서는 안 됩니다. 제3자 상표 또는 로고의 사용은 해당 권리자의 정책을 따릅니다.

</div>

<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**포크 및 번역 안내** 이 저장소는 [microsoft/RustTraining](https://github.com/microsoft/RustTraining)을 기반으로 한 포크입니다. 현재 README는 한국어로 번역되었으며, 이후 문서도 순차적으로 번역될 수 있습니다. 이 번역본은 비공식 자료이며 Microsoft의 승인, 후원 또는 공식 번역이 아닙니다. 원문 출처와 라이선스 고지는 유지됩니다.

</div>

# Rust 학습서 모음

다양한 프로그래밍 배경을 가진 개발자를 위한 Rust 학습서 7권과, async, 고급 패턴, 엔지니어링 실무를 다루는 심화 자료를 모아둔 저장소입니다.

이 자료는 원본 콘텐츠에 더해 Rust 생태계의 뛰어난 자료들에서 영감을 받은 아이디어와 예제를 함께 엮어 구성되었습니다. 목표는 여러 책, 블로그, 발표, 영상에 흩어져 있는 지식을 교육용 커리큘럼으로 재구성해, 깊이 있고 기술적으로 정확한 학습 경험을 제공하는 것입니다.

> **주의:** 이 책들은 학습 자료이며 공식 레퍼런스가 아닙니다. 정확성을 위해 노력하고 있지만, 중요한 내용은 항상 [공식 Rust 문서](https://doc.rust-lang.org/)와 [Rust Reference](https://doc.rust-lang.org/reference/)로 다시 확인하세요.

### 참고 및 감사

- [**The Rust Programming Language**](https://doc.rust-lang.org/book/) - 전체 커리큘럼의 기반이 된 핵심 자료
- [**Jon Gjengset**](https://www.youtube.com/c/JonGjengset) - 고급 Rust 내부 동작을 다루는 심화 스트림과 `Crust of Rust`
- [**withoutboats**](https://without.boats/blog/) - async 설계, `Pin`, futures 모델에 대한 통찰
- [**fasterthanlime (Amos)**](https://fasterthanli.me/) - 시스템 프로그래밍을 원리부터 풀어내는 장문 탐구
- [**Mara Bos**](https://marabos.nl/) - *Rust Atomics and Locks*를 통한 동시성 프리미티브 설명
- [**Aleksey Kladov (matklad)**](https://matklad.github.io/) - rust-analyzer, API 설계, 에러 처리 패턴에 대한 통찰
- [**Niko Matsakis**](https://smallcultfollowing.com/babysteps/) - 언어 설계, borrow checker 내부 구조, Polonius 관련 글
- [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/) 및 [**Rustonomicon**](https://doc.rust-lang.org/nomicon/) - 실전 예제와 unsafe Rust 심화 자료
- [**This Week in Rust**](https://this-week-in-rust.org/) - 다양한 예제와 아이디어에 영향을 준 커뮤니티 소식
- [**Binary Musings - Tag(Rust)**](https://binarymusings.org/posts/category/rust/) - Rust 내부 동작을 깊게 다루는 글
- 그 외에도 수많은 **Rust 커뮤니티**의 블로그 글, 컨퍼런스 발표, RFC, 포럼 토론이 이 자료에 큰 영향을 주었습니다. 모두 일일이 적기 어렵지만 깊이 감사드립니다.

## 📖 읽기 시작하기

본인의 배경에 맞는 책부터 선택하면 됩니다. 책들은 난이도와 목적에 따라 분류되어 있어 학습 경로를 잡기 쉽습니다.

| 수준 | 설명 |
|-------|-------------|
| 🟢 **Bridge** | 다른 언어 경험을 바탕으로 Rust에 입문하기 좋은 책 |
| 🔵 **Deep Dive** | Rust의 특정 핵심 영역을 깊게 파고드는 책 |
| 🟡 **Advanced** | 숙련된 Rust 개발자를 위한 패턴과 기법 |
| 🟣 **Expert** | 타입 수준 정확성과 고급 모델링 기법 |
| 🟤 **Practices** | 엔지니어링, 툴링, 운영 실무 중심 |

| 책 | 수준 | 추천 대상 |
|------|-------|-------------|
| [**C/C++ 개발자를 위한 Rust**](https://burnnnnny.github.io/RustTraining/c-cpp-book/) | 🟢 Bridge | move semantics, RAII, FFI, embedded, `no_std`에 익숙한 개발자 |
| [**C# 개발자를 위한 Rust**](https://burnnnnny.github.io/RustTraining/csharp-book/) | 🟢 Bridge | Swift / C# / Java 배경에서 ownership과 타입 시스템을 배우려는 개발자 |
| [**Python 개발자를 위한 Rust**](https://burnnnnny.github.io/RustTraining/python-book/) | 🟢 Bridge | 동적 타이핑에서 정적 타이핑과 GIL 없는 동시성으로 넘어가려는 개발자 |
| [**Async Rust**](https://burnnnnny.github.io/RustTraining/async-book/) | 🔵 Deep Dive | Tokio, stream, cancellation safety를 깊게 이해하려는 개발자 |
| [**Rust Patterns**](https://burnnnnny.github.io/RustTraining/rust-patterns-book/) | 🟡 Advanced | Pin, allocator, lock-free 구조, unsafe 패턴을 다루려는 개발자 |
| [**타입 주도 정확성**](https://burnnnnny.github.io/RustTraining/type-driven-correctness-book/) | 🟣 Expert | typestate, phantom type, capability token에 관심 있는 개발자 |
| [**Rust 엔지니어링 실무**](https://burnnnnny.github.io/RustTraining/engineering-book/) | 🟤 Practices | build script, cross-compilation, CI/CD, Miri 등 실무 역량을 다지고 싶은 개발자 |

각 책은 15~16개 정도의 장으로 구성되어 있으며, Mermaid 다이어그램, 수정 가능한 Rust playground, 연습 문제, 전문 검색 기능을 포함합니다.

> **팁:** 사이드바 탐색과 검색이 포함된 렌더링 문서는 [GitHub Pages endpoint](https://burnnnnny.github.io/RustTraining/)에서 볼 수 있습니다.
>
> **로컬 미리보기:** 오프라인으로 읽거나 기여하면서 확인하려면 [Rust](https://rustup.rs/)를 설치한 뒤 아래처럼 실행하세요.
> ```
> git clone https://github.com/Burnnnnny/RustTraining.git
> cd RustTraining
> cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
> cargo xtask serve    # http://localhost:3000
> ```

---

## 🔧 유지보수자를 위한 안내

<details>
<summary>로컬에서 책을 빌드, 서빙, 수정하는 방법</summary>

### 사전 준비

아직 Rust가 없다면 [**rustup**](https://rustup.rs/)으로 설치한 뒤, 아래 도구를 설치하세요.

```bash
cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
```

### 저장소 클론

```bash
git clone https://github.com/Burnnnnny/RustTraining.git
cd RustTraining
```

### 빌드 및 실행

```bash
cargo xtask build               # 모든 책을 site/에 빌드
cargo xtask serve               # 빌드 후 http://localhost:3000 에서 서빙
cargo xtask deploy              # 모든 책을 docs/에 빌드
cargo xtask clean               # site/ 및 docs/ 정리
```

특정 책 하나만 빌드하거나 서빙하려면:

```bash
cd c-cpp-book && mdbook serve --open    # http://localhost:3000
```

### 배포

`main` 브랜치에 push하면 `.github/workflows/pages.yml`이 `cargo xtask deploy`를 실행하고, 생성된 `docs/` 산출물을 GitHub Pages에 배포합니다. 배포 endpoint는 `https://burnnnnny.github.io/RustTraining/` 형식을 사용합니다.

</details>

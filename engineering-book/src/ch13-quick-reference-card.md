<a id="quick-reference-card"></a>
# 빠른 레퍼런스 카드

<a id="cheat-sheet-commands-at-a-glance"></a>
### 한눈에 보는 명령 치트시트

```bash
# ─── 빌드 스크립트 ───
cargo build                          # build.rs를 먼저 컴파일한 뒤 크레이트
cargo build -vv                      # 자세히 — build.rs 출력 표시

# ─── 크로스 컴파일 ───
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17
cross build --release --target aarch64-unknown-linux-gnu

# ─── 벤치마킹 ───
cargo bench                          # 모든 벤치마크 실행
cargo bench -- parse                 # "parse"에 맞는 벤치마크만
cargo flamegraph -- --args           # 바이너리에서 flamegraph 생성
perf record -g ./target/release/bin  # perf 데이터 기록
perf report                          # perf 데이터 대화형 보기

# ─── 커버리지 ───
cargo llvm-cov --html                # HTML 리포트
cargo llvm-cov --lcov --output-path lcov.info
cargo llvm-cov --workspace --fail-under-lines 80
cargo tarpaulin --out Html           # 대체 도구

# ─── 안전 검증 ───
cargo +nightly miri test             # Miri 아래에서 테스트 실행
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test
valgrind --leak-check=full ./target/debug/binary
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu

# ─── 감사 및 공급망 ───
cargo audit                          # 알려진 취약점 스캔
cargo audit --deny warnings          # 권고가 있으면 CI 실패
cargo deny check                     # 라이선스 + 권고 + 금지 + 소스 검사
cargo deny list                      # 의존성 트리의 모든 라이선스 나열
cargo vet                            # 공급망 신뢰 검증
cargo outdated --workspace           # 오래된 의존성 찾기
cargo semver-checks                  # 깨지는 API 변경 탐지
cargo geiger                         # 의존성 트리의 unsafe 개수

# ─── 바이너리 최적화 ───
cargo bloat --release --crates       # 크레이트별 크기 기여
cargo bloat --release -n 20          # 가장 큰 함수 20개
cargo +nightly udeps --workspace     # 미사용 의존성 찾기
cargo machete                        # 빠른 미사용 의존성 탐지
cargo expand --lib module::name      # 매크로 확장 결과 보기
cargo msrv find                      # 최소 Rust 버전 찾기
cargo clippy --fix --workspace --allow-dirty  # 린트 자동 수정

# ─── 컴파일 시간 최적화 ───
export RUSTC_WRAPPER=sccache         # 공유 컴파일 캐시
sccache --show-stats                 # 캐시 히트 통계
cargo nextest run                    # 더 빠른 테스트 러너
cargo nextest run --retries 2        # 불안정한 테스트 재시도

# ─── 플랫폼 엔지니어링 ───
cargo check --target thumbv7em-none-eabihf   # no_std 빌드 검증
cargo build --target x86_64-pc-windows-gnu   # Windows용 크로스 컴파일
cargo xwin build --target x86_64-pc-windows-msvc  # MSVC ABI 크로스 컴파일
cfg!(target_os = "linux")                    # 컴파일 타임 cfg(bool로 평가)

# ─── 릴리스 ───
cargo release patch --dry-run        # 릴리스 미리보기
cargo release patch --execute        # 버전 올리고 커밋, 태그, 게시
cargo dist plan                      # 배포 아티팩트 미리보기
```

<a id="decision-table-which-tool-when"></a>
### 의사결정 표: 언제 어떤 도구

| 목표 | 도구 | 쓸 때 |
|------|------|--------|
| git 해시 / 빌드 정보 식립 | `build.rs` | 바이너리에 추적 가능성이 필요할 때 |
| Rust로 C 코드 컴파일 | `build.rs`의 `cc` 크레이트 | 작은 C 라이브러리 FFI |
| 스키마에서 코드 생성 | `prost-build` / `tonic-build` | Protobuf, gRPC, FlatBuffers |
| 시스템 라이브러리 링크 | `build.rs`의 `pkg-config` | OpenSSL, libpci, systemd |
| 정적 Linux 바이너리 | `--target x86_64-unknown-linux-musl` | 컨테이너/클라우드 배포 |
| 오래된 glibc 타깃 | `cargo-zigbuild` | RHEL 7, CentOS 7 호환 |
| ARM 서버 바이너리 | `cross` 또는 `cargo-zigbuild` | Graviton/Ampere 배포 |
| 통계적 벤치마크 | Criterion.rs | 성능 회귀 탐지 |
| 빠른 성능 점검 | Divan | 개발 중 프로파일링 |
| 핫스팟 찾기 | `cargo flamegraph` / `perf` | 벤치마크로 느린 코드를 찾은 뒤 |
| 줄/분기 커버리지 | `cargo-llvm-cov` | CI 커버리지 게이트, 공백 분석 |
| 빠른 커버리지 점검 | `cargo-tarpaulin` | 로컬 개발 |
| Rust UB 탐지 | Miri | 순수 Rust `unsafe` 코드 |
| C FFI 메모리 안전 | Valgrind memcheck | Rust/C 혼합 코드베이스 |
| 데이터 레이스 탐지 | TSan 또는 Miri | 동시 `unsafe` 코드 |
| 버퍼 오버플로 탐지 | ASan | `unsafe` 포인터 산술 |
| 누수 탐지 | Valgrind 또는 LSan | 장기 실행 서비스 |
| 로컬 CI에 가깝게 | `cargo-make` | 개발자 워크플로 자동화 |
| 프리커밋 검사 | `cargo-husky` 또는 git hook | 푸시 전에 문제 차단 |
| 자동 릴리스 | `cargo-release` + `cargo-dist` | 버전 관리 + 배포 |
| 의존성 감사 | `cargo-audit` / `cargo-deny` | 공급망 보안 |
| 라이선스 준수 | `cargo-deny`(licenses) | 상용 / 엔터프라이즈 프로젝트 |
| 공급망 신뢰 | `cargo-vet` | 고보안 환경 |
| 오래된 의존성 찾기 | `cargo-outdated` | 정기 유지보수 |
| 깨지는 변경 탐지 | `cargo-semver-checks` | 라이브러리 크레이트 게시 |
| 의존성 트리 분석 | `cargo tree --duplicates` | 중복 제거 및 그래프 정리 |
| 바이너리 크기 분석 | `cargo-bloat` | 크기 제약 배포 |
| 미사용 의존성 찾기 | `cargo-udeps` / `cargo-machete` | 컴파일 시간과 크기 줄이기 |
| LTO 조정 | `lto = true` 또는 `"thin"` | 릴리스 바이너리 최적화 |
| 크기 최적화 바이너리 | `opt-level = "z"` + `strip = true` | 임베디드 / WASM / 컨테이너 |
| unsafe 사용 감사 | `cargo-geiger` | 보안 정책 준수 |
| 매크로 디버깅 | `cargo-expand` | Derive / macro_rules 디버깅 |
| 더 빠른 링킹 | `mold` 링커 | 개발자 내부 루프 |
| 컴파일 캐시 | `sccache` | CI와 로컬 빌드 속도 |
| 더 빠른 테스트 | `cargo-nextest` | CI와 로컬 테스트 속도 |
| MSRV 준수 | `cargo-msrv` | 라이브러리 게시 |
| `no_std` 라이브러리 | `#![no_std]` + `default-features = false` | 임베디드, UEFI, WASM |
| Windows 크로스 컴파일 | `cargo-xwin` / MinGW | Linux → Windows 빌드 |
| 플랫폼 추상화 | `#[cfg]` + 트레잇 패턴 | 멀티 OS 코드베이스 |
| Windows API 호출 | `windows-sys` / `windows` 크레이트 | 네이티브 Windows 기능 |
| 종단 간 타이밍 | `hyperfine` | 전체 바이너리 벤치마크, 전후 비교 |
| 속성 기반 테스트 | `proptest` | 엣지 케이스, 파서 견고성 |
| 스냅샷 테스트 | `insta` | 큰 구조화 출력 검증 |
| 커버리지 유도 퍼징 | `cargo-fuzz` | 파서에서 충돌 탐색 |
| 동시성 모델 검사 | `loom` | 락프리 자료구조, 원자 순서 |
| feature 조합 테스트 | `cargo-hack` | 여러 `#[cfg]` feature가 있는 크레이트 |
| 빠른 UB 검사(거의 네이티브) | `cargo-careful` | CI 안전 게이트, Miri보다 가벼움 |
| 저장 시 자동 재빌드 | `cargo-watch` | 개발자 내부 루프, 빠른 피드백 |
| 워크스페이스 문서 | `cargo doc` + rustdoc | API 탐색, 온보딩, doc-link CI |
| 재현 가능 빌드 | `--locked` + `SOURCE_DATE_EPOCH` | 릴리스 무결성 검증 |
| CI 캐시 조정 | `Swatinem/rust-cache@v2` | 빌드 시간 단축(콜드 → 캐시) |
| 워크스페이스 린트 정책 | Cargo.toml의 `[workspace.lints]` | 모든 크레이트에 일관된 Clippy/컴파일러 린트 |
| 린트 자동 수정 | `cargo clippy --fix` | 사소한 문제 자동 정리 |

<a id="further-reading"></a>
### 추가 읽을거리

| 주제 | 자료 |
|------|------|
| Cargo 빌드 스크립트 | [Cargo Book — Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) |
| 크로스 컴파일 | [Rust Cross-Compilation](https://rust-lang.github.io/rustup/cross-compilation.html) |
| `cross` 도구 | [cross-rs/cross](https://github.com/cross-rs/cross) |
| `cargo-zigbuild` | [cargo-zigbuild docs](https://github.com/rust-cross/cargo-zigbuild) |
| Criterion.rs | [Criterion User Guide](https://bheisler.github.io/criterion.rs/book/) |
| Divan | [Divan docs](https://github.com/nvzqz/divan) |
| `cargo-llvm-cov` | [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) |
| `cargo-tarpaulin` | [tarpaulin docs](https://github.com/xd009642/tarpaulin) |
| Miri | [Miri GitHub](https://github.com/rust-lang/miri) |
| Rust의 Sanitizer | [rustc Sanitizer docs](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/sanitizer.html) |
| `cargo-make` | [cargo-make book](https://sagiegurari.github.io/cargo-make/) |
| `cargo-release` | [cargo-release docs](https://github.com/crate-ci/cargo-release) |
| `cargo-dist` | [cargo-dist docs](https://axodotdev.github.io/cargo-dist/book/) |
| 프로파일 유도 최적화 | [Rust PGO guide](https://doc.rust-lang.org/rustc/profile-guided-optimization.html) |
| Flamegraph | [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) |
| `cargo-deny` | [cargo-deny docs](https://embarkstudios.github.io/cargo-deny/) |
| `cargo-vet` | [cargo-vet docs](https://mozilla.github.io/cargo-vet/) |
| `cargo-audit` | [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit) |
| `cargo-bloat` | [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) |
| `cargo-udeps` | [cargo-udeps](https://github.com/est31/cargo-udeps) |
| `cargo-geiger` | [cargo-geiger](https://github.com/geiger-rs/cargo-geiger) |
| `cargo-semver-checks` | [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) |
| `cargo-nextest` | [nextest docs](https://nexte.st/) |
| `sccache` | [sccache](https://github.com/mozilla/sccache) |
| `mold` 링커 | [mold](https://github.com/rui314/mold) |
| `cargo-msrv` | [cargo-msrv](https://github.com/foresterre/cargo-msrv) |
| LTO | [rustc Codegen Options](https://doc.rust-lang.org/rustc/codegen-options/index.html) |
| Cargo 프로파일 | [Cargo Book — Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) |
| `no_std` | [Rust Embedded Book](https://docs.rust-embedded.org/book/) |
| `windows-sys` 크레이트 | [windows-rs](https://github.com/microsoft/windows-rs) |
| `cargo-xwin` | [cargo-xwin docs](https://github.com/rust-cross/cargo-xwin) |
| `cargo-hack` | [cargo-hack](https://github.com/taiki-e/cargo-hack) |
| `cargo-careful` | [cargo-careful](https://github.com/RalfJung/cargo-careful) |
| `cargo-watch` | [cargo-watch](https://github.com/watchexec/cargo-watch) |
| Rust CI 캐시 | [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) |
| Rustdoc 책 | [Rustdoc Book](https://doc.rust-lang.org/rustdoc/) |
| 조건부 컴파일 | [Rust Reference — cfg](https://doc.rust-lang.org/reference/conditional-compilation.html) |
| 임베디드 Rust | [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust) |
| `hyperfine` | [hyperfine](https://github.com/sharkdp/hyperfine) |
| `proptest` | [proptest](https://github.com/proptest-rs/proptest) |
| `insta` | [insta snapshot testing](https://insta.rs/) |
| `cargo-fuzz` | [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) |
| `loom` | [loom concurrency testing](https://github.com/tokio-rs/loom) |

---

*동반 자료로 만든 레퍼런스 — Rust Patterns와 타입 주도 정확성 자료의 companion입니다.*

*버전 1.3 — cargo-hack, cargo-careful, cargo-watch, cargo doc, 재현 가능 빌드, CI 캐시 전략, 캡스톤 연습, 완성도를 위한 장 의존성 다이어그램을 추가했습니다.*

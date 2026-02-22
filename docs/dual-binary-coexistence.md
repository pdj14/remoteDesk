# RemoteDesk 두 바이너리 공존 구조 제안

## 1. 목표
- `remotedesk.exe`는 기존처럼 **GUI 원격 제어** 기능에 집중한다.
- `remotedesk-cli.exe`는 **콘솔(CMD/PowerShell) 친화적 CLI** 기능에 집중한다.
- 두 바이너리는 같은 코드베이스/설정을 공유하여 동작 불일치를 줄인다.

## 2. 왜 분리하는가
- Windows GUI 서브시스템(`windows_subsystem = "windows"`) EXE는 콘솔 출력/대기 동작이 환경에 따라 일관적이지 않다.
- 반대로 콘솔 서브시스템 EXE로 통합하면 GUI 실행 시 콘솔창 처리 이슈가 생긴다.
- 따라서 GUI/CLI 책임 분리가 가장 안정적이다.

## 3. 산출물
- `remotedesk.exe`
  - 용도: 화면 보기/원격 제어/설정 UI/트레이/서비스 연동
  - 동작: 현재와 동일하게 GUI 중심
- `remotedesk-cli.exe`
  - 용도: `--version`, `--get-id`, 운영/설정 명령 자동화
  - 동작: 콘솔에서 즉시 출력, 프로세스 대기/종료코드 명확

## 4. 기능 분리 기준
- GUI 전용
  - 기본 실행(인자 없음)
  - 원격 화면 표시/제어 세션
  - UI 기반 설정 변경
- CLI 전용 또는 우선
  - `--version`, `--build-date`, `--get-id`
  - `--option`, `--set-id`, `--password` 같은 운영 명령
  - 스크립트/배치 자동화 시나리오

## 5. 권장 아키텍처
- 공통 명령 처리 모듈을 분리한다.
  - 예: `src/cli_commands.rs`
  - 역할: 인자 파싱, 권한/사전조건 체크, 결과/종료코드 반환
- 각 진입점은 최소 책임만 가진다.
  - `src/main.rs` (`remotedesk.exe`): GUI/기존 로직
  - `src/main_cli.rs` (`remotedesk-cli.exe`): 공통 명령 모듈 호출 + stdout/stderr 출력
- 핵심 원칙
  - 비즈니스 로직은 공유
  - 출력/대기 방식만 바이너리별로 분리

## 6. Cargo/빌드 구성(개념)
- `Cargo.toml`에 CLI 바이너리 타깃 추가
  - `[[bin]] name = "remotedesk-cli" path = "src/main_cli.rs"`
- Windows에서 `main_cli.rs`는 콘솔 서브시스템 유지
  - GUI 서브시스템 속성 미적용
- 패키징 스크립트는 2개 EXE를 함께 배포
  - `remotedesk.exe`
  - `remotedesk-cli.exe`

## 7. 사용자 사용 예시
- GUI 실행
  - `remotedesk.exe`
- CLI 확인
  - `remotedesk-cli.exe --version`
  - `remotedesk-cli.exe --get-id`
- 스크립트
  - `for /f %%i in ('remotedesk-cli.exe --get-id') do set RDID=%%i`

## 8. 호환성 정책
- 기존 사용자 습관 보호
  - `remotedesk.exe`의 기존 GUI 동작은 변경하지 않는다.
- 점진적 전환
  - 문서/스크립트/자동화는 CLI 바이너리로 안내한다.
- 필요 시 호환 브리지
  - `remotedesk.exe --version` 같은 일부 호출은 유지 가능(권장 경로는 `remotedesk-cli.exe`)

## 9. 테스트 체크리스트
- `remotedesk.exe` GUI 정상 실행/세션 연결/화면 표시
- `remotedesk-cli.exe --version` 콘솔 즉시 출력
- `remotedesk-cli.exe --get-id > file` 리다이렉션 정상
- 관리자 권한 필요 명령의 종료코드/에러메시지 일관성
- 서비스/트레이와 동시 실행 시 충돌 없음

## 10. 기대 효과
- GUI 안정성 유지
- CLI 자동화 신뢰성 향상
- 운영 문서/배치 스크립트 단순화

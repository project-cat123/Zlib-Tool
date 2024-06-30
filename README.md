# Zlib Compress / Decompress
폐쇄망에서 CS 프로그램 분석을 위한 툴
프록시 기능 추가할 예정이며 상황에 따라 Burp extender로 방향성 바뀔 수 있음

# Compiling
필요 :
러스트 컴파일러

```
git clone https://github.com/project-cat123/Zlib-Tool.git

cd Zlib-Tool
```

### cargo
```
cargo build
cargo run
```
---

# 사용법
## Home
### 압축
1. 압축할 데이터 입력 후 Compress
![image](https://github.com/project-cat123/Zlib-Tool/assets/78392616/9236c3ff-ef03-4938-8ae2-67c3ff3ad87f)

2. 16진수로 압축된 값 출력 확인
![image](https://github.com/project-cat123/Zlib-Tool/assets/78392616/45bc3887-fb5f-43e7-911b-eba5dea313b7)


---
### 압축 해제
1. 헥스 값 중 압축데이터의 헤더에 해당하는 78 9C 이후 데이터 복사
![image](https://github.com/project-cat123/Zlib-Tool/assets/78392616/c9060e0d-a3d5-48b9-89ca-5a66d1ff6d09)

2. 복사한 값을 넣은 후 Decompress
![image](https://github.com/project-cat123/Zlib-Tool/assets/78392616/1e48aa85-646c-4030-be86-7356577c1811)

3. 압축 해제 데이터 확인
![image](https://github.com/project-cat123/Zlib-Tool/assets/78392616/34be5882-5e6e-401c-af5c-9d9c4bc9d270)


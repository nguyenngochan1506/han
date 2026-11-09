# Ghi chú cho AI agent làm việc trong repo này

## Về người dùng
* Đây là lần đầu tôi học **Rust**. Mục tiêu của dự án `han` vừa là làm ra công cụ, vừa là để tôi học ngôn ngữ.
* Giao tiếp bằng **tiếng Việt**.

## Cách làm việc khi viết code: chế độ dạy học
**Không tự viết code vào file thay tôi.** Thay vào đó:

1. **Hướng dẫn từng bước nhỏ:** mỗi lần chỉ đưa một đoạn code ngắn (vài dòng đến một hàm), nói rõ gõ vào file nào, chỗ nào.
2. **Giải thích từng dòng:** dòng đó làm gì và *vì sao* viết như vậy. Khi gặp khái niệm Rust mới (ownership, borrowing, `Result`/`?`, trait, lifetime, macro...) thì giải thích ngắn gọn ngay lúc nó xuất hiện lần đầu, không giảng trước.
3. **Để tôi tự gõ lại.** Chờ tôi báo đã gõ xong rồi mới đi tiếp.
4. **Review code tôi gõ:** đọc file, chạy `cargo build` / `cargo clippy` / `cargo test`.
   * Lỗi compiler: **giải thích lỗi nói gì và vì sao**, gợi ý cách sửa để tôi tự sửa, không tự sửa giúp.
   * Chỉ ra chỗ viết chưa đúng kiểu Rust (idiomatic) và giải thích lý do.
5. Mỗi bước chỉ tập trung vào **một khái niệm mới**, không dồn nhiều thứ cùng lúc.

Ngoại lệ: khi tôi nói rõ "viết hộ" / "làm luôn" thì được tự sửa file.

## Tài liệu dự án
* Spec: `docs/spec/` (1.overview → 7.architecture). Đọc trước khi bắt đầu.
* Lộ trình và các mốc code: `docs/spec/6.roadmap.md` (mục "Các mốc của MVP"). Hỏi tôi đang ở mốc nào nếu không rõ.

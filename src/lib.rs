// src/lib.rs

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// JSに公開する関数 (テストで呼び出すため pub が必要)
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    let result = a + b;
    alert(&format!("Rust Wasm: {} + {} = {}", a, b, result));
    result
}


#[cfg(test)]
mod wasm_tests {
    use wasm_bindgen_test::*;
    use super::*; // ★★★ add関数をインポートするために必須

    // run_in_browser は通常不要ですが、ブラウザAPIを使う場合は必要
    wasm_bindgen_test_configure!(run_in_browser); 

    #[wasm_bindgen_test]
    fn test_add_function() {
        assert_eq!(add(10, 5), 15, "10 + 5 は 15 であるべき");
    }

    // 非同期テストのコードは一旦コメントアウトまたは削除
    /*
    #[wasm_bindgen_test]
    async fn test_async_operation() {
        // let result = wasm_add_async(20, 30).await;
        // assert_eq!(result, 50);
    }
    */
}
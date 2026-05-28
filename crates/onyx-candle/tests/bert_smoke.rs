use onyx_candle::models::bert::{BertModelBuilder, BertResourceConfig};
use onyx_tokio::io::TokioReader;
use onyx_tokio::net::TokioResolver;

#[tokio::test]
#[ignore = "downloads ~90MB from huggingface.co; run with `cargo test --features http,json -- --ignored`"]
async fn bert_mini_lm_encode_smoke() {
    let model = BertModelBuilder::new()
        .resources(BertResourceConfig::mini_lm_l6_v2())
        .reader(TokioReader::default())
        .resolver(TokioResolver::default())
        .build()
        .await
        .expect("build BertModel");

    let hidden = model.encode("Hello, world!").expect("encode");

    let dims = hidden.dims();
    assert_eq!(dims.len(), 3, "expected [batch, seq, hidden], got {dims:?}");
    assert_eq!(dims[0], 1, "batch dim");
    assert_eq!(dims[2], 384, "MiniLM-L6 hidden size");
    assert_eq!(hidden.dtype(), candle_core::DType::F32);

    let sample: f32 = hidden.flatten_all().unwrap().get(0).unwrap().to_scalar().unwrap();
    assert!(sample.is_finite(), "first element should be finite, got {sample}");
}

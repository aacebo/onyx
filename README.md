# Onyx

A high-performance, ONNX-first machine learning inference runtime for Rust.

Built for production systems, Onyx provides a modern Rust-native interface for running transformer models and NLP pipelines without Python, libtorch, or heavyweight runtime dependencies.

Onyx focuses on:

* ONNX-native model execution
* High-throughput inference
* Pipeline-oriented APIs
* Production observability
* Efficient batching and scheduling
* Portable deployment across CPU and GPU backends

Unlike traditional ML frameworks designed primarily for research workflows, Onyx is designed for systems engineers building real-world inference infrastructure.

## Features

* ONNX Runtime backend
* Transformer pipeline support
* Token classification / NER
* Sentence embeddings
* Sequence classification
* Streaming and batched inference
* Hugging Face-compatible tokenizer support
* Async-safe runtime architecture
* Zero-Python production deployment
* Modular workspace design

## Goals

Onyx aims to provide a robust foundation for:

* NLP services
* AI agents
* Embedding infrastructure
* Semantic search
* Annotation pipelines
* Distributed inference systems
* Edge and server-side model execution

## Philosophy

Onyx is not a training framework.

It is an inference runtime focused on correctness, performance, portability, and operational simplicity.

```text id="6o1tsw"
Train anywhere -> Export to ONNX -> Run with Onyx
```

## Status

Onyx is currently under active development.

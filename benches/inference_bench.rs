use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

// Estruturas mockadas simulando o comportamento real do loader e motor de inferência
// para permitir a compilação e teste isolado da suite de benchmarking.
struct DummyEngine {
    layers_loaded: usize,
}

impl DummyEngine {
    fn new() -> Self {
        Self { layers_loaded: 0 }
    }

    fn simulate_lazy_load_eval(&mut self, token_id: u32) -> f32 {
        // Simula a busca sequencial e leitura de camadas via MMAP de forma agressiva
        self.layers_loaded = (token_id % 32) as usize;
        let mut accumulator = 0.0f32;
        for i in 0..self.layers_loaded {
            accumulator += (i as f32).sin();
        }
        accumulator
    }
}

fn bench_token_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("Thasky Core Inference");
    
    // Configurações críticas para hardware limitado: medições precisas sem bater teto térmico
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));
    group.sample_size(50);

    let mut engine = DummyEngine::new();
    let token_input: u32 = 420;

    // Teste 1: Latência pura por avaliação de token isolado
    group.bench_with_input(
        BenchmarkId::new("lazy_load_token_eval", token_input),
        &token_input,
        |b, &input| {
            b.iter(|| engine.simulate_lazy_load_eval(input));
        },
    );

    group.finish();
}

fn bench_kv_cache_eviction(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Management");
    
    let mut mock_kv_cache = vec![0.0f32; 4096]; // Janela cheia simulada

    // Teste 2: Velocidade do algoritmo de limpeza de contexto (Eviction) sob estresse de RAM
    group.bench_function("kv_cache_flash_eviction_4k", |b| {
        b.iter(|| {
            // Simula redução cirúrgica de 4096 para 1024 tokens ativos (truncamento de cache)
            mock_kv_cache.truncate(1024);
            if mock_kv_cache.len() < 1024 {
                mock_kv_cache.resize(4096, 0.0f32);
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_token_latency, bench_kv_cache_eviction);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use awtrix3::{Color, Notification};

fn color_parsing_benchmark(c: &mut Criterion) {
    c.bench_function("color_from_hex", |b| {
        b.iter(|| Color::from_hex(black_box("#FF0000")))
    });
    
    c.bench_function("color_to_hex", |b| {
        let color = Color::RED;
        b.iter(|| black_box(&color).to_hex())
    });
}

fn notification_building_benchmark(c: &mut Criterion) {
    c.bench_function("notification_builder", |b| {
        b.iter(|| {
            Notification::builder()
                .text("Test notification")
                .icon(1234)
                .color(Color::RED)
                .duration(10)
                .build()
        })
    });
}

fn serialization_benchmark(c: &mut Criterion) {
    let notification = Notification::builder()
        .text("Test notification")
        .icon(1234)
        .color(Color::RED)
        .duration(10)
        .build();
    
    c.bench_function("notification_serialization", |b| {
        b.iter(|| serde_json::to_string(black_box(&notification)))
    });
}

criterion_group!(
    benches,
    color_parsing_benchmark,
    notification_building_benchmark,
    serialization_benchmark
);
criterion_main!(benches);
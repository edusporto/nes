use criterion::{criterion_group, criterion_main, Criterion};
use nes::{cartridge::Cartridge, Nes};

pub fn fps_no_render(c: &mut Criterion) {
    let mut nes = Nes::new(Cartridge::from_file("test_data/roms/nestest.nes").unwrap());

    c.bench_function("Run 1 frame", |b| {
        b.iter(|| {
            nes.next_frame();
        })
    });
}

criterion_group!(benches, fps_no_render);
criterion_main!(benches);

// fn fill_by_index(v: &mut AsnNodeView2d) {
//     let mut index: u8 = 0;
//     for y in 0..MAP_VIEW_SIZE.height {
//         for x in 0..MAP_VIEW_SIZE.width {
//             v.set_cell(&Pos2D { x, y }, index).unwrap();
//             index = index.wrapping_add(1);
//         }
//     }
// }

// fn randomize_view(mut rng: SmallRng, v: &mut AsnNodeView2d) -> SmallRng {
//     for x in 0..MAP_VIEW_SIZE.width {
//         for y in 0..MAP_VIEW_SIZE.height {
//             let c: u8 = rng.gen_range(0..128);
//             v.set_cell(&Pos2D { x, y }, c).unwrap();
//         }
//     }
//     rng
// }

// fn randomize_array(mut rng: SmallRng, a: &mut Array2D<u32, u8>) -> SmallRng {
//     let x = rng.gen_range(0..a.size.width);
//     let y = rng.gen_range(0..a.size.height);
//
//     let byte = rng.gen_range(0..3);
//
//     let index = y * a.size.width * 4 + x * 4 + byte;
//
//     let cell: u8 = rng.gen_range(0..255);
//     a.bytes[index as usize] = cell;
//     for x in 0..a.size.width {
//         for y in 0..a.size.height {
//             let index = ((y * a.size.width + x) * 4) as usize;
//             a.bytes[index] = rng.gen_range(0..128);
//             a.bytes[index + 1] = rng.gen_range(0..128);
//             a.bytes[index + 2] = rng.gen_range(0..128);
//             a.bytes[index + 3] = rng.gen_range(0..128);
//         }
//     }
//     rng
// }

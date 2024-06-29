use std::process::Command;

use ab_glyph::{FontRef, PxScale};
use imageproc::{
    drawing::{draw_hollow_circle, draw_hollow_circle_mut, draw_text_mut, text_size},
    image::{GenericImageView, ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage},
};
use papier::papervm::{MemoryCell, PaperVM, Pos};

struct FinishedPaper {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    answer: (u64, u64, u64, u64),
}

pub fn collect_papers<T: MemoryCell>(root: PaperVM<T>) -> Vec<PaperVM<T>> {
    let mut papers = vec![root.clone()];
    for paper in root.finished_papers {
        papers.extend(collect_papers(paper));
    }
    papers
}

pub fn render_papers<T: MemoryCell>(root: PaperVM<T>) {
    for (i, papier) in collect_papers(root).into_iter().enumerate() {
        let paper = render_paper(&papier);
        paper.save(format!("papier_{}.png", i)).unwrap();
    }
}

fn render_paper<T: MemoryCell>(vm: &PaperVM<T>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let memory = vm.get_memory();
    let mut lines = vec![];

    for y in 0..100 {
        let mut line = vec![];
        for x in 0..75 {
            let ch = memory
                .get(&Pos(x, y))
                .map(|c| c.read().to_string())
                .unwrap_or(" ".to_string());
            line.push(ch);
        }
        lines.push(line.concat().to_string());
    }

    let circled = vm.get_circled().unwrap();

    let mut paper = RgbaImage::new(2000, 2000);
    paper.fill(255);
    let font = FontRef::try_from_slice(include_bytes!("../JH2TRIAL.ttf")).unwrap();

    let text = lines.join("\n");

    let mut w = 0;
    let mut h = 20;

    for (i, line) in lines.into_iter().enumerate() {
        draw_text_mut(
            &mut paper,
            Rgba([0u8, 0u8, 0u8, 255u8]),
            20,
            h as i32,
            PxScale::from(48.0),
            &font,
            &line,
        );
        let (next_w, next_h) = text_size(PxScale::from(48.0), &font, &line);
        w = w.max(next_w);
        h += next_h + 6;

        if i == circled.0 .1 as usize {
            println!("w,h: {next_w} {h}");
            println!("CIRCLE: {circled}");
            let circle_x = circled.0 .0 as i32 * 12 + circled.1 as i32;
            let circle_y = h as i32;
            println!("result: {circle_x} {circle_y}");
            draw_hollow_circle_mut(
                &mut paper,
                (circle_x, circle_y),
                circled.1 as i32 * 3,
                Rgba([0u8, 0u8, 0u8, 255u8]),
            );
        }
    }

    println!("Text size: {}x{}", w, h);

    paper.view(0, 0, 20 + w, 20 + h + 200).to_image()
}

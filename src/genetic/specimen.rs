use crate::{genetic::gene::Gene, utils::canvas::Canvas, utils::reference::Reference};
use num::traits::Pow;
use rayon::prelude::*;

#[derive(Clone)]
pub struct Specimen {
    genes: Vec<Gene>,
    pub score: u32,
    pub similitude: f32,
}

fn draw_score_from_genes(reference: &Reference, genes: &Vec<Gene>) -> Canvas {
    let mut canvas = Canvas::from_canvas(&reference.canvas);
    for gene in genes.iter() {
        canvas.draw_circle(gene.x, gene.y, gene.radius, gene.color);
    }
    canvas
}

fn retrieve_score_from_genes(reference: &Reference, genes: &Vec<Gene>) -> (u32, f32) {
    let canvas = draw_score_from_genes(reference, genes);
    let similitude = canvas.compare(&reference.canvas);
    let score = similitude.pow(2) as u32;
    (score, similitude)
}

impl Specimen {
    pub fn new(reference: &Reference) -> Specimen {
        let genes: Vec<Gene> = (0..2000)
            .into_par_iter()
            .map(|_| Gene::new(reference))
            .collect();

        let (score, similitude) = retrieve_score_from_genes(reference, &genes);

        Specimen {
            genes,
            score,
            similitude,
        }
    }

    pub fn make_love(&self, partner: &Specimen, reference: &Reference) -> Specimen {
        let parent_genes = self.genes.par_iter().zip(partner.genes.par_iter());
        let genes = parent_genes
            .map(|(father_gene, mother_gene)| father_gene.crossover(mother_gene, reference))
            .collect();
        let (score, similitude) = retrieve_score_from_genes(reference, &genes);

        Specimen {
            genes,
            score,
            similitude,
        }
    }

    pub fn render(&self, reference: &Reference) -> Canvas {
        draw_score_from_genes(reference, &self.genes)
    }
}

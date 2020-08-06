mod utils;

use wasm_bindgen::prelude::*;
use std::collections::HashSet;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Gene {
    G,
    Y,
    H,
    W,
    X,
}

#[derive(Debug, Eq, PartialEq)]
struct GeneSet {
    g: i32,
    y: i32,
    h: i32,
    w: i32,
    x: i32
}

impl GeneSet {
    fn plant_matches(&self, plant: &[Gene; 6]) -> bool {
        let mut plant_geneset = GeneSet{g: 0, y: 0, h: 0, w: 0, x: 0};
        for gene in plant {
            match gene {
                Gene::G => plant_geneset.g += 1,
                Gene::Y => plant_geneset.y += 1,
                Gene::H => plant_geneset.h += 1,
                Gene::W => plant_geneset.w += 1,
                Gene::X => plant_geneset.x += 1,
            }
        }

        &plant_geneset == self
    }
}

#[derive(Debug)]
struct GeneAdder {
    g: i32,
    y: i32,
    h: i32,
    w: i32,
    x: i32,
}

impl GeneAdder {
    fn gene_result(&self) -> Vec<Gene> {
        let mut possible_genes = Vec::new();
        let mut genes = [(Gene::G, self.g), (Gene::Y, self.y), (Gene::H, self.h), (Gene::W, self.w), (Gene::X, self.x)];
        genes.sort_by_key(|k| k.1);
        genes.reverse();

        let mut top_score = 0;
        for (gene, score) in genes.iter() {
            if *score < top_score {
                break;
            }
            top_score = *score;
            possible_genes.push(*gene);
        }

        possible_genes
    }
}

const GOOD_GENE: i32 = 6;
const BAD_GENE: i32 = 8;

fn expand_gene_tree(tree: Vec<Vec<Gene>>, plant: &mut [Gene; 6], index: usize, plants: &mut Vec<[Gene; 6]>) {
    if index == 6 {
        plants.push(*plant);
        return;
    }

    for possible_gene in tree[index].iter() {
        plant[index] = *possible_gene;
        expand_gene_tree(tree.clone(), plant, index+1, plants);
    }
}

fn combine_genes(plants: Vec<&[Gene; 6]>) -> Vec<[Gene; 6]> {

    let mut output_plant_tree = Vec::new();
    let mut output_plants = Vec::new();

    for i in 0..6 {
        let mut gene_sums = GeneAdder{g: 0, y: 0, h: 0, w: 0, x: 0};
        for plant in plants.iter() {
            match plant[i] {
                Gene::G => gene_sums.g += GOOD_GENE,
                Gene::Y => gene_sums.y += GOOD_GENE,
                Gene::H => gene_sums.h += GOOD_GENE,
                Gene::W => gene_sums.w += BAD_GENE,
                Gene::X => gene_sums.x += BAD_GENE,
            }

        }
        output_plant_tree.push(gene_sums.gene_result());
    }

    let mut tmp_plant = [Gene::X; 6];
    expand_gene_tree(output_plant_tree, &mut tmp_plant, 0, &mut output_plants);

    output_plants
}

fn crossbreed_generation(goal: &GeneSet, plants: HashSet<[Gene; 6]>) -> Option<String> {
    let mut best_combo: Option<String> = None;
    let mut best_combo_size = 1000;

    'outer: for plant1 in &plants {
        for plant2 in &plants {
            for plant3 in &plants {
                for plant4 in &plants {
                    let possible_children = combine_genes(vec![plant1, plant2, plant3, plant4]);
                    if possible_children.len() < best_combo_size {
                        for child in &possible_children {
                            if goal.plant_matches(child) {
                                best_combo = Some(format!("  {:?}\n  {:?}\n  {:?}\n  {:?}\n= {:?} {:.2}%", plant1, plant2, plant3, plant4, child, 1.0/(possible_children.len() as f32) * 100.0));
                                best_combo_size = possible_children.len();
                                if possible_children.len() == 1 {
                                    break 'outer;
                                }
                            }
                        }
                    }

                }
            }
        }
    }

    best_combo
}

#[wasm_bindgen]
pub fn solve(g: i32, y: i32, h: i32, w: i32, x: i32, plant_txt: &str) -> String {
    utils::set_panic_hook();
    if g + y + h + w + x != 6 {
        return "Number of genes in goal does not add to 6".to_string();
    }

    let valid_gene_re = regex::Regex::new(r"^[GYHWX]{6}$").unwrap();

    let  goal = GeneSet{g, y, h, w, x};

    let mut plants = HashSet::new();
    for line in plant_txt.lines() {
        let line = line.to_ascii_uppercase();
        if !valid_gene_re.is_match(&line) {
            return format!("Invalid Gene Set: {}", line).to_string();
        }
        let mut new_plant = [Gene::X; 6];
        for (i, c) in line.chars().enumerate() {
            let gene = match c {
                'G' => Gene::G,
                'Y' => Gene::Y,
                'H' => Gene::H,
                'W' => Gene::W,
                'X' => Gene::X,
                _ => panic!("invalid gene")
            };
            new_plant[i] = gene;
        }
        plants.insert(new_plant);
    }

    match crossbreed_generation(&goal, plants) {
        Some(output) => output,
        None => String::from("Unable to solve")
    }
}

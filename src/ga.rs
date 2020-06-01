use rand::Rng;
const CROSSOVER_PROB: f64 = 0.5;
const MUTATION_PROB: f64 = 0.1;

pub struct Population<T: Gene + Clone> {
    pub chromosomes: Vec<Chromosome<T>>,
    score_sum: f64,
}

#[derive(Debug, Clone)]
pub struct Chromosome<T: Gene + Clone> {
    pub genes: Vec<T>,
    pub score: f64,
}

pub trait Gene {
    fn mutate(&mut self);
}

impl<T: Gene + Clone> Population<T> {
    pub fn new() -> Population<T> {
        Population {
            chromosomes: Vec::new(),
            score_sum: 0.0,
        }
    }

    pub fn fittness(&mut self, eval: &dyn Fn(&Chromosome<T>) -> f64) {
        self.score_sum = 0.0;
        for i in 0..self.chromosomes.len() {
            if self.chromosomes[i].score == 0.0 {
                self.chromosomes[i].fittness(eval);
            }
            self.score_sum += self.chromosomes[i].score;
        }
        self.chromosomes
            .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    }

    pub fn next_gen(
        &mut self,
        cross_over: &dyn Fn(&Chromosome<T>, &Chromosome<T>) -> (Chromosome<T>, Chromosome<T>),
    ) {
        let size = self.chromosomes.len();
        assert_eq!(
            size % 2,
            0,
            "population size {} must be divisible by 2",
            size
        );

        let mut next_gen = Vec::with_capacity(size);
        next_gen.push(self.chromosomes[0].clone());
        next_gen.push(self.chromosomes[1].clone());

        let weights = weights(&self.chromosomes);

        for _ in 0..((size - 2) / 2) {
            let (parent_1, parent_2) = select_parents(&self.chromosomes, &weights);
            let (mut child_1, mut child_2) = gen_children(parent_1, parent_2, cross_over);
            child_1.mutate();
            child_2.mutate();
            next_gen.push(child_1);
            next_gen.push(child_2);
        }

        self.chromosomes = next_gen;
    }
}

impl<T: Gene + Clone> Chromosome<T> {
    fn fittness(&mut self, eval: &dyn Fn(&Chromosome<T>) -> f64) {
        self.score = eval(self);
    }

    fn mutate(&mut self) {
        for i in 0..self.genes.len() {
            if rand::thread_rng().gen_bool(MUTATION_PROB) {
                self.genes[i].mutate();
            }
        }
    }
}

fn weights<T: Gene + Clone>(chromosomes: &Vec<Chromosome<T>>) -> Vec<f64> {
    let mut sum = 0.0;
    let mut weights = vec![sum];

    for value in chromosomes {
        sum += value.score;
        weights.push(sum);
    }
    weights
}

fn select_parents<'a, T: Gene + Clone>(
    chromosomes: &'a Vec<Chromosome<T>>,
    weights: &Vec<f64>,
) -> (&'a Chromosome<T>, &'a Chromosome<T>) {
    let index_1 = roulette_selection(weights);
    let index_2 = roulette_selection(weights);

    (&chromosomes[index_1], &chromosomes[index_2])
}

fn roulette_selection(weights: &Vec<f64>) -> usize {
    let weight_sum = weights.last().unwrap();
    let rnd: f64 = rand::thread_rng().gen_range(0.0, weight_sum);

    weights.iter().rposition(|&weight| weight < rnd).unwrap()
}

fn gen_children<T: Gene + Clone>(
    parent_1: &Chromosome<T>,
    parent_2: &Chromosome<T>,
    f: &dyn Fn(&Chromosome<T>, &Chromosome<T>) -> (Chromosome<T>, Chromosome<T>),
) -> (Chromosome<T>, Chromosome<T>) {
    if rand::thread_rng().gen_bool(CROSSOVER_PROB) {
        f(parent_1, parent_2)
    } else {
        (parent_1.clone(), parent_2.clone())
    }
}

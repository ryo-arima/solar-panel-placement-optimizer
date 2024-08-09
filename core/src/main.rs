use rand::prelude::*;

//GA版テスト
//パネルA (ma)kg,(Sa)m2
//パネルB (mb)kg,(Sb)m2
//...

//トラックA (Ea)km/L or km/kWh, max(Ma)kg (陸拠点), 速度v(km/h)
//トラックB (Eb)km/L or km/kWh, max(Mb)kg (陸拠点), 速度v(km/h)
//...

//船A (Ea)km/L or km/kWh, max(Ma)kg, 速度v(km/h)
//船B (Eb)km/L or km/kWh, max(Mb)kg, 速度v(km/h)
//...

//鉄道A km/kWh, max(Ma)kg, 速度v(km/h)
//鉄道B km/kWh, max(Mb)kg, 速度v(km/h)
//...

//バッテリーA (Ba)Ah, (Bma)kg, 自己放電率(%)
//バッテリーB (Bb)Ah, (Bmb)kg, 自己放電率(%)
//...

//拠点A 充電電流(C-BAa)A, 予想消費電力W, スペース（バッテリー Sm2 x n層、パネル Sm2、船舶 Sm2、トラック Sm2）
//拠点B 充電電流(C-BAb)B, 予想消費電力W, スペース（バッテリー Sm2 x n層、パネル Sm2、船舶 Sm2、トラック Sm2）
//...

//拠点間距離AB(Rab)km

//計算単位 1週間ごとの配置、天気予報（日照時間）、日照時間（角度）
// https://ssl4.eir-parts.net/doc/4418/tdnet/2332520/00.pdf
// https://www.jstage.jst.go.jp/article/jsmedsd/2017.27/0/2017.27_2112/_pdf
// https://www.nedo.go.jp/news/press/AA5_101648.html

//雛形
#[derive(Clone, Debug)]
struct Individual {
    genes: Vec<i32>,
    fitness: i32,
}

impl Individual {
    fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let genes: Vec<i32> = (0..size).map(|_| rng.gen_range(0..10)).collect();
        let fitness = genes.iter().sum();
        Individual { genes, fitness }
    }

    fn crossover(&self, partner: &Individual) -> Individual {
        let mut rng = rand::thread_rng();
        let crossover_point = rng.gen_range(0..self.genes.len());
        let mut child_genes = self.genes[0..crossover_point].to_vec();
        child_genes.extend_from_slice(&partner.genes[crossover_point..]);
        let fitness = child_genes.iter().sum();
        Individual {
            genes: child_genes,
            fitness,
        }
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let mutation_point = rng.gen_range(0..self.genes.len());
        self.genes[mutation_point] = rng.gen_range(0..10);
        self.fitness = self.genes.iter().sum();
    }
}

fn select(population: &mut Vec<Individual>) -> &Individual {
    population.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    &population[0]
}

fn genetic_algorithm(population_size: usize, generations: usize, gene_size: usize) -> Individual {
    let mut population: Vec<Individual> = (0..population_size)
        .map(|_| Individual::new(gene_size))
        .collect();

    for _ in 0..generations {
        let parent1 = select(&mut population).clone();
        let parent2 = select(&mut population).clone();

        let mut child = parent1.crossover(&parent2);
        child.mutate();

        population.push(child);
    }

    select(&mut population).clone()
}

fn main() {
    let best_individual = genetic_algorithm(100, 1000, 10);
    println!("Best individual: {:?}", best_individual);
}

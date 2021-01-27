/*
https://en.wikipedia.org/wiki/K-means_clustering
Goal reduce within-cluster sum of squares = varaince
 */

use rand::prelude::ThreadRng;
use rand::seq::IteratorRandom;

use crate::points::{Closest, Dist, Mean, Point};

pub trait KMeans {
    fn group(&self, points: &[Point], group_count: usize) -> Vec<Vec<Point>>;
}

/*
https://en.wikipedia.org/wiki/K-means_clustering#Standard_algorithm_(na%C3%AFve_k-means)
 */
struct NaiveKMeans {
    rng: ThreadRng,
    iterations: usize,
}

impl NaiveKMeans {
    fn new(rdm: ThreadRng, iterations: usize) -> NaiveKMeans {
        NaiveKMeans {
            rng: rdm,
            iterations,
        }
    }
}

impl KMeans for NaiveKMeans {
    fn group(&self, points: &[Point], group_count: usize) -> Vec<Vec<Point>> {
        let mut rng = self.rng.clone();

        let mut groups = vec![vec![]; group_count];

        let mut centroids = points
            .iter()
            .cloned()
            .choose_multiple(&mut rng, group_count);

        for _ in 0..self.iterations {
            groups.iter_mut().for_each(|group| group.clear());
            for point in points {
                // closest centroid
                let closest_idx = centroids.closest_idx(point);

                // add to group
                groups[closest_idx].push(*point);
            }
            centroids = groups.iter().map(|group| group.mean()).collect();
        }
        groups
    }
}

#[cfg(test)]
mod kmeans_test {
    use std::collections::HashSet;

    use rand::prelude::ThreadRng;

    use crate::k_means::{KMeans, NaiveKMeans};
    use crate::points::Point;
    use rand::seq::SliceRandom;

    #[test]
    fn test_kmeans_circle() {
        let len1: usize = 10;
        let len2: usize = 30;
        let mut circle_one: Vec<_> = (0..len1)
            .map(|_| Point::circle_rand(0.0, 0.0, 3.0))
            .collect();

        let mut circle_two: Vec<_> = (0..len2)
            .map(|_| Point::circle_rand(10.0, 10.0, 5.0))
            .collect();


        let mut points = Vec::with_capacity(circle_one.len() + circle_two.len());
        points.append(&mut circle_one);
        points.append(&mut circle_two);
        points.shuffle(&mut ThreadRng::default());

        let k_means = NaiveKMeans::new(ThreadRng::default(), 10_000);
        let res = k_means.group(&points, 2);

        let counts: HashSet<_> = res.iter().map(|x| x.len()).collect();

        println!("counts {:?}", counts);
        assert!(counts.contains(&len1));
        assert!(counts.contains(&len2));
    }
}

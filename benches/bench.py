import numpy as np
from alphashape import alphashape

from fast_alphashape import alphashape as fast_alphashape


def create_data() -> np.ndarray:
    return np.random.uniform(low=-90.0, high=90.0, size=(1_000, 2))


def test_benchmark_fast_alphashape(benchmark):
    data = create_data()
    benchmark(fast_alphashape, data, alpha=1.0)


def test_benchmark_alphashape(benchmark):
    data = create_data()
    benchmark(alphashape, data, alpha=1.0)

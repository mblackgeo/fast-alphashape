from pathlib import Path

import numpy as np
import pytest


@pytest.fixture
def resources() -> Path:
    return Path(__file__).parent / "resources"


@pytest.fixture
def points() -> np.ndarray:
    return np.array(
        [
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.5, 0.25),
            (0.5, 0.75),
            (0.25, 0.5),
            (0.75, 0.5),
        ]
    )

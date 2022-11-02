import numpy as np
from fast_alphashape import __version__, alpha_shape
from shapely import wkt


def test_version():
    assert __version__ is not None


def test_basic_2d_shape():
    points_2d = np.array(
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
    res = alpha_shape(points_2d, 2.0)
    expect = wkt.loads(
        "POLYGON ((0.75 0.5, 1 0, 0.5 0.25, 0 0, 0.25 0.5, 0 1, 0.5 0.75, 1 1, 0.75 0.5))"
    )

    assert res.equals(expect)

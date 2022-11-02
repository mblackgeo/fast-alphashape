import numpy as np
from shapely import wkt

from fast_alphashape import __version__, alphashape


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
    res = alphashape(points_2d, 2.0)
    expect = wkt.loads(
        "POLYGON ((0.75 0.5, 1 0, 0.5 0.25, 0 0, 0.25 0.5, 0 1, 0.5 0.75, 1 1, 0.75 0.5))"
    )

    assert res.equals(expect)

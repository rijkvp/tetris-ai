# This is a simplified version of the original code from https://github.com/clsibert/Tetris-AI
# which was used as a reference for this version in Rust.
# Changed are marked with CHANGE comments.
# All credit goes to the original authors.

from copy import deepcopy
import collections.abc
import functools
import itertools
import numpy as np
import random
import time


# Returns a function that converts an implementation into features with the given dependencies.
#
# In terms of features, an implementation is any callable object that accepts at least one argument.
# The first positional argument refers to the state to be analyzed, while additional arguments refer
# to the evaluations of declared dependencies on the state.
#
# Features themselves are wrapper functions around the implementations they were created from.
# These wrappers accept a state to pass along to the implementation, along with an optional cache to
# consult before computing features.
# As a result, each feature will only be computed once when the evaluation calls share a cache.
def define(*needs):
    def wrap(impl, name=None):
        assert callable(impl)

        if name is not None:
            impl.__name__ = name

        # Wrap the feature implementation to utilize a cache and gather evaluated dependencies
        @functools.wraps(impl)
        def feature(state, cache=None):
            if cache is None:
                cache = {}
            feature.accesses += 1
            if impl in cache:
                result = cache[impl]
            else:
                feature.misses += 1
                result = impl(state, *(need(state, cache=cache) for need in needs))
                cache[impl] = result

            return result

        feature.accesses = 0
        feature.misses = 0
        return feature

    return wrap


# Returns a new feature that evaluates the given feature on a transformed state.
def with_transformed_state(transformer, feature):
    # Does not pass on cache, since a feature may be evaluated on different states
    return lambda state, cache=None: feature(transformer(state))


# Evaluates a collection of features on a given state.
# If features is a Mapping, then it assumed to map features to corresponding weights.
# The results are returned via dict, where each feature is mapped to its result.
def evaluate(state, features):
    cache = {}
    if isinstance(features, collections.abc.Mapping):
        return {f: f(state, cache=cache) * w for (f, w) in features.items()}
    else:
        return {f: f(state, cache=cache) for f in features}


# Height Features


# __heights:
#   The height of each of the board' columns, as a list indexed by column.
#   NOTE: This is a helper feature, and should only be used for computing other features.
@define()
def __heights(state):
    return [state.board.height(c) for c in range(0, state.board.cols())]


# Transition Features


# col_trans:
#   The number of times that two adjacent cells in the same column mismatch.
@define()
def col_trans(state):
    return np.sum(np.diff(state.board.data, axis=0))


# row_trans:
#   The number of times that two adjacent cells in the same row mismatch.
@define()
def row_trans(state):
    return np.sum(np.diff(state.board.data, axis=1))


# __wells:
#   The depth of each column with respect to its adjacent columns, as a list.
#   If a column is not shorter than both of its neighbors, it has a value of 0.
#   Otherwise, its value is how much shorter it is than its shortest neighbor.
#   NOTE: This is a helper feature, and should only be used for computing other features.
@define(__heights)
def __wells(state, __heights):
    __heights = [state.board.rows()] + __heights + [state.board.rows()]
    return [
        max(0, min(__heights[i - 1], __heights[i + 1]) - __heights[i])
        for i in range(1, len(__heights) - 1)
    ]


# cuml_wells:
#   The sum from 1 to wells.
#   Computed using the formula for the sum of natural numbers.
@define(__wells)
def cuml_wells(_, __wells):
    # CHANGE: use integer division to avoid floating point arithmetic
    return sum(x * (x + 1) // 2 for x in __wells)


# pit features


# __pits:
#   A collection of empty cells with filled cells above them, as a list.
#   Each element is the set of rows in which a well is found for the column corresponding to the
#   element's index.
#   NOTE: This is a helper feature, and should only be used for computing other features.
@define(__heights)
def __pits(state, __heights):
    return [
        {
            r
            for r in range(state.board.rows() - __heights[c] + 1, state.board.rows())
            if not state.board[r, c]
        }
        for c in range(0, state.board.cols())
    ]


# pits:
#   The total number of pits.
@define(__pits)
def pits(_, __pits):
    return sum(len(rows) for rows in __pits)


# Miscellaneous Features


# landing_height:
#   The height of the row containing the bottom-most cell of the previously placed zoid.
@define()
def landing_height(state):
    return state.board.rows() - (
        state.delta.row + state.delta.zoid[state.delta.rot].shape[0]
    )


# eroded_cells:
#   The number of cells that were cleared from the previously placed zoid.
@define()
def eroded_cells(state):
    return sum(
        sum(state.delta.zoid[state.delta.rot][row - state.delta.row])
        for row in state.delta.cleared
    )


class Zoid:
    def __init__(self, name, shape, rots):
        self.name = name
        self.shapes = tuple(np.rot90(shape, k=-i) for i in range(0, rots))

    def __repr__(self):
        return "Zoid.{}".format(self.name)

    def __len__(self):
        return len(self.shapes)

    def __getitem__(self, rot):
        return self.shapes[rot]


def enum(**kwargs):
    return collections.namedtuple("enum", kwargs)(**kwargs)


zoids = enum(
    classic=enum(
        I=Zoid("I", np.array([[1, 1, 1, 1]], dtype=np.bool_), 2),
        T=Zoid("T", np.array([[1, 1, 1], [0, 1, 0]], dtype=np.bool_), 4),
        L=Zoid("L", np.array([[1, 1, 1], [1, 0, 0]], dtype=np.bool_), 4),
        J=Zoid("J", np.array([[1, 1, 1], [0, 0, 1]], dtype=np.bool_), 4),
        O=Zoid("O", np.array([[1, 1], [1, 1]], dtype=np.bool_), 1),
        Z=Zoid("Z", np.array([[1, 1, 0], [0, 1, 1]], dtype=np.bool_), 2),
        S=Zoid("S", np.array([[0, 1, 1], [1, 1, 0]], dtype=np.bool_), 2),
    ),
)

del enum


class Board:
    def __init__(self, rows, cols, zero=True):
        if zero:
            self.data = np.zeros((rows, cols), dtype=np.bool_)
            self.heights = [0] * cols
        else:
            self.data = np.empty((rows, cols), dtype=np.bool_)

    def rows(self):
        return self.data.shape[0]

    def cols(self):
        return self.data.shape[1]

    def row(self, row):
        return self.data[row]

    def col(self, col):
        return self.data[::, col]

    def height(self, col):
        return self.heights[col]

    def imprint(self, orient, row, col):
        self.data[row : row + orient.shape[0], col : col + orient.shape[1]] |= orient
        for c in range(0, orient.shape[1]):
            # CHANGE: fix empty columns
            nz = np.nonzero(orient[::, c])[0]
            if len(nz) == 0:
                continue
            top = self.rows() - row - nz[0]
            if top >= self.heights[col + c]:
                self.heights[col + c] = top

    def overlaps(self, orient, row, col):
        if row < 0 or col < 0:
            return True
        elif row + orient.shape[0] > self.rows() or col + orient.shape[1] > self.cols():
            return True
        else:
            return np.any(
                self.data[row : row + orient.shape[0], col : col + orient.shape[1]]
                & orient
            )

    def full(self):
        return {r for (r, full) in enumerate(np.all(self.data, axis=1)) if full}

    def clear(self, rows):
        self.data = np.insert(
            np.delete(self.data, list(rows), 0),
            0,
            np.zeros((len(rows), self.cols()), dtype=np.bool_),
            axis=0,
        )
        for c in range(0, self.cols()):
            nonzero = np.nonzero(self.col(c))[0]
            if len(nonzero) > 0:
                self.heights[c] = self.rows() - nonzero[0]
            else:
                self.heights[c] = 0

    def __getitem__(self, pos):
        return self.data[pos]

    def __repr__(self):
        return "{}x{} Board@{:#x}".format(self.rows(), self.cols(), id(self))

    def __str__(self):
        return "\n".join(
            [
                "".join(["#" if self[i, j] else "." for j in range(0, self.cols())])
                for i in range(0, self.rows())
            ]
        )

    def __eq__(self, other):
        return np.array_equal(self.data, other.data)

    def __ne__(self, other):
        return not np.array_equal(self.data, other.data)

    def __deepcopy__(self, memo):
        clone = Board(self.rows(), self.cols(), zero=False)
        np.copyto(clone.data, self.data)
        clone.heights = list(self.heights)
        return clone


class State:
    class Delta:
        def __init__(self, zoid, rot, row, col, cleared):
            self.zoid = zoid
            self.rot = rot
            self.row = row
            self.col = col
            self.cleared = cleared

    def __init__(self, prev, board, cleared=None, delta=None):
        self.prev = prev
        self.board = board
        self.cleared = cleared
        self.delta = delta

        if self.cleared is None:
            self.cleared = collections.defaultdict(int)

    def lines_cleared(self, simultaneously=None):
        if simultaneously is not None:
            return self.cleared[simultaneously]

        return sum(count * times for (count, times) in self.cleared.items())

    def level(self):
        return self.lines_cleared() // 10

    def score(self, points=(0, 40, 100, 300, 1200)):
        score = 0
        while self.delta is not None:
            score += points[len(self.delta.cleared)] * (self.level() + 1)
            self = self.prev
        return score

    def future(self, zoid, rot, row, col):
        # Copy important info from this state
        board = deepcopy(self.board)
        cleared = self.cleared.copy()

        # Compute future info with copies
        board.imprint(zoid[rot], row, col)
        full = frozenset(board.full())
        delta = State.Delta(zoid, rot, row, col, full)
        if len(full) > 0:
            cleared[len(full)] += 1
            board.clear(full)

        # Create future state with this as the previous one
        return State(self, board, cleared=cleared, delta=delta)


def move_drop(state, zoid):
    board = state.board
    for rot in range(0, len(zoid)):
        orient = zoid[rot]
        for col in range(0, board.cols() - orient.shape[1] + 1):
            highest = max(board.height(col + c) for c in range(0, orient.shape[1]))
            row = board.rows() - highest - orient.shape[0]
            if not board.overlaps(orient, row, col):
                while not board.overlaps(orient, row + 1, col):
                    row += 1
                yield (rot, row, col)


def policy_best(scorer, tie_breaker):
    # Like builtin max, but returns all elements that have max value
    def all_max(itr, key):
        group = []
        max = None
        for e in itr:
            k = key(e)
            if max is None or max < k:
                # e is new max, since previous max is strictly less
                group = []
                max = k
            elif k < max:
                # e cannot be a max, so skip it
                continue
            # k >= max
            # e is a max, so add to group
            group.append(e)
        return group

    def pick_best(futures):
        maxes = all_max(futures, scorer)
        if len(maxes) == 0:
            return None
        elif len(maxes) == 1:
            return maxes[0]
        else:
            return tie_breaker(maxes)

    return pick_best


def simulate(state, zoid_gen, move_gen, move_policy, lookahead=1):
    assert lookahead > 0
    while True:
        # Slice out 'visible' zoids using lookahead
        zoids = tuple(itertools.islice(zoid_gen, 0, lookahead))
        # Create a stack of future pools
        # First layer is simply the current state
        pool_stack = [[state]]

        # Fill remaining pool layers by generating possible futures from previous layer
        for zoid in zoids:
            pool_stack.append(
                [
                    prev.future(zoid, *move)
                    for prev in pool_stack[-1]
                    for move in move_gen(prev, zoid)
                ]
            )

        # Apply the move policy to the farthest non-empty future pool
        next = None
        while next is None and len(pool_stack) > 1:
            pool = pool_stack.pop()
            if pool:
                next = move_policy(pool)

        if next is not None:
            # Backtrack to find first move on the path to this far-future, and yield it
            while next.prev is not state:
                next = next.prev
            yield next

            # Reset for the next iteration
            state = next
            # Push all yet-unplaced zoids back into front of zoid_gen, if any
            if len(zoids) > 1:
                zoid_gen = itertools.chain(zoids[1:], zoid_gen)
        else:
            # There are no possible moves, so the simulation is over
            break


testfeatures = {
    landing_height: -3.383,
    eroded_cells: -9.277,
    row_trans: -2.700,
    col_trans: -6.786,
    pits: -12.668,
    cuml_wells: -0.396,
}


## Create a piece generator (can use a list here, or a generator function)
def zoid_gen(zoids, rng):
    while True:
        yield rng.choice(zoids)


# Added for testing purposes
def import_state(board_data):
    board = Board(20, 10)
    matrix = np.array(board_data, dtype=np.bool_)
    board.imprint(matrix, 0, 0)
    return State(None, board)


if __name__ == "__main__":
    move_gen = move_drop  # Create a move generator (this is where overhang detection or time pressure filtering are implemented. move_drop is basic version)
    feats = testfeatures  # Define Features
    state = State(None, Board(20, 10))  ## Define Initial State
    seed = 101  ## Pick a seed (if using)
    start_time = time.time()
    sim = simulate(
        state,
        zoid_gen(zoids.classic, random.Random(seed)),
        move_gen,
        policy_best(
            lambda state: sum(evaluate(state, feats).values()),
            random.Random(-seed).choice,
        ),
        2,
    )  ## Create simulator "object"

    ## Run the simulator
    # Inside this loop, print episode level data to file (take a good look at state and delta classes in game.py)
    for episode, state in enumerate(sim, 1):
        # print(episode, state.delta.zoid)
        # print(state.board)

        # set some kind of limit
        if episode >= 10:
            break

    elapsed = time.time() - start_time
    print(
        f"episodes: {episode}, elapsed =  {elapsed:.2f}s, epi/s =  {episode/elapsed:.2f}"
    )
    print("score: ", state.score())
    print(state.board)

    print(state.board.heights)
    print(state.board.data)
    print(state.lines_cleared(1))
    print(state.lines_cleared(2))
    print(state.lines_cleared(3))
    print(state.lines_cleared(4))

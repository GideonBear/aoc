from __future__ import annotations

from enum import Enum


class Choice(Enum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3

    @classmethod
    def from_letter(cls, letter: str) -> Choice:
        return CHOICE_LETTER_MAP[letter]

    @classmethod
    def from_opponent_endstate(cls, opponent: Choice, endstate: End) -> Choice:
        if endstate == End.DRAW:
            return opponent
        elif endstate == End.LOSS:
            return BEAT_MAP[opponent]
        else:
            return LOSS_MAP[opponent]


Round = tuple[Choice, Choice]


CHOICE_LETTER_MAP = {
    'A': Choice.ROCK,
    'B': Choice.PAPER,
    'C': Choice.SCISSORS,
}
BEAT_MAP = {
    Choice.ROCK: Choice.SCISSORS,
    Choice.PAPER: Choice.ROCK,
    Choice.SCISSORS: Choice.PAPER,
}
LOSS_MAP = {
    v: k for k, v in BEAT_MAP.items()
}


class End(Enum):
    WIN = 6
    DRAW = 3
    LOSS = 0

    @classmethod
    def from_letter(cls, letter: str) -> End:
        return END_LETTER_MAP[letter]


END_LETTER_MAP = {
    'X': End.LOSS,
    'Y': End.DRAW,
    'Z': End.WIN,
}


def score_from_round(round: Round) -> int:
    opponent, end = round
    you = Choice.from_opponent_endstate(opponent, end)
    choicescore = you.value
    winscore = end.value
    return choicescore + winscore


with open('2.txt') as f:
    text = f.read().strip('\n')


rounds = (
    (Choice.from_letter(round[0]), End.from_letter(round[2]))
    for round in text.split('\n')
)

total = sum(score_from_round(round) for round in rounds)
print(total)

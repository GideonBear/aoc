from __future__ import annotations

from enum import Enum


class Choice(Enum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3

    @classmethod
    def from_letter(cls, letter: str) -> Choice:
        return LETTER_MAP[letter]

    def winscore(self, opponent: Choice) -> int:
        if opponent == self:
            return DRAW
        elif BEAT_MAP[self] == opponent:
            return WIN
        else:
            return LOSS


Round = tuple[Choice, Choice]


LETTER_MAP = {
    'A': Choice.ROCK,
    'B': Choice.PAPER,
    'C': Choice.SCISSORS,
    'X': Choice.ROCK,
    'Y': Choice.PAPER,
    'Z': Choice.SCISSORS,
}
BEAT_MAP = {
    Choice.ROCK: Choice.SCISSORS,
    Choice.PAPER: Choice.ROCK,
    Choice.SCISSORS: Choice.PAPER,
}
WIN = 6
DRAW = 3
LOSS = 0


def score_from_round(round: Round) -> int:
    opponent, you = round
    choicescore = you.value
    winscore = you.winscore(opponent)
    return choicescore + winscore


with open('2.txt') as f:
    text = f.read().strip('\n')


rounds = (
    (Choice.from_letter(choice)
     for choice in round.split(' ')
     ) for round in text.split('\n')
)

total = sum(score_from_round(round) for round in rounds)
print(total)

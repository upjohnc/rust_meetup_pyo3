class Math_:
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def add(self):
        return self.left + self.right

    def mult(self):
        return self.left * self.right


def concat_(left: str, right: str) -> str:
    return f"{left}-{right}"


if __name__ == "__main__":
    m = Math_(3, 6)
    print(f"Mulitplication: {m.mult()}")
    print(f"Addition: {m.add()}")

    print(f"Concat: {concat_('lefty', 'righty')}")

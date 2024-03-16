from rust_meetup_pyo3 import Math_, concat_


if __name__ == "__main__":
    m = Math_(3, 6)
    print(f"Mulitplication: {m.mult()}")
    print(f"Addition: {m.add()}")

    print(f"Concat: {concat_('lefty', 'righty')}")

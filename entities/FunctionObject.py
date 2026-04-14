class FunctionObject:
    def __init__(self, name, file):
        self.name = name
        self.file = file

    def __str__(self):
        return f"FunctionObject(name={self.name}, file={self.file})"
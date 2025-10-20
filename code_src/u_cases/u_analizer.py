from desreference import PointerDereference as ptrDref, PointerAssignment as ptrAsgn


class UnsafeAnalyzer:

    types_of_unsafe = [
        ptrDref,
        ptrAsgn,
    ]


    def analyze(self, code_block, start_line):
        for tu in self.types_of_unsafe:
            instance = tu.match(code_block, start_line)
            if instance:
                return instance